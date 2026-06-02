//! JIT (Just-In-Time) context discovery for subdirectories.
//!
//! When tools access files in subdirectories, this module automatically
//! discovers and loads AGENTS.md / CLAUDE.md / instructions.md files from
//! those subdirectories, providing context-aware guidance to the agent.

use std::path::{Path, PathBuf};

use crate::project_context::{
    load_project_context, ProjectContext,
};

/// Discovered JIT context for a path
#[derive(Debug, Clone)]
pub struct JitContext {
    /// The directory where context was found
    pub directory: PathBuf,
    /// The loaded context content
    pub context: ProjectContext,
}

/// Discover JIT context for a given path by checking the path's directory
/// and all ancestor directories up to (but not including) the workspace root.
///
/// This allows subdirectory-specific AGENTS.md files to provide context
/// when the agent works in that subdirectory.
///
/// # Arguments
/// * `accessed_path` - The file or directory path being accessed
/// * `workspace_root` - The workspace root (search stops here, not included)
///
/// # Returns
/// The discovered context, or `None` if no subdirectory context was found.
pub fn discover_jit_context(accessed_path: &Path, workspace_root: &Path) -> Option<JitContext> {
    let dir = if accessed_path.is_dir() {
        accessed_path.to_path_buf()
    } else {
        accessed_path.parent()?.to_path_buf()
    };

    // Walk from the accessed directory up to (but not including) workspace root
    let mut current = Some(dir.as_path());
    while let Some(check_dir) = current {
        // Stop if we've reached the workspace root
        if check_dir == workspace_root {
            break;
        }

        // Check for context files in this directory
        let ctx = load_project_context(check_dir);
        if ctx.has_instructions() {
            return Some(JitContext {
                directory: check_dir.to_path_buf(),
                context: ctx,
            });
        }

        current = check_dir.parent();
    }

    None
}

/// Format discovered JIT context as a string suitable for appending
/// to tool output or system prompt.
pub fn format_jit_context(jit: &JitContext) -> String {
    let dir_display = jit
        .directory
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("subdirectory");

    let source_display = jit
        .context
        .source_path
        .as_ref()
        .map(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("AGENTS.md")
        })
        .unwrap_or("AGENTS.md");

    let content = jit
        .context
        .instructions
        .as_deref()
        .unwrap_or("");

    format!(
        "\n\n--- JIT Context: {dir_display}/{source_display} ---\n\
         {content}\n\
         --- End JIT Context ---"
    )
}

/// Discover JIT context for multiple paths (batch operation).
/// Returns deduplicated contexts, ordered by specificity (deepest first).
pub fn discover_jit_contexts_batch(
    paths: &[&Path],
    workspace_root: &Path,
) -> Vec<JitContext> {
    let mut seen_dirs = std::collections::HashSet::new();
    let mut results = Vec::new();

    for path in paths {
        if let Some(jit) = discover_jit_context(path, workspace_root) {
            if seen_dirs.insert(jit.directory.clone()) {
                results.push(jit);
            }
        }
    }

    // Sort by path depth (deepest first = most specific)
    results.sort_by(|a, b| {
        b.directory
            .components()
            .count()
            .cmp(&a.directory.components().count())
    });

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_discover_jit_context_no_file() {
        let tmp = tempdir().unwrap();
        let workspace = tmp.path();
        let sub = workspace.join("subdir");
        fs::create_dir(&sub).unwrap();

        let result = discover_jit_context(&sub, workspace);
        assert!(result.is_none());
    }

    #[test]
    fn test_discover_jit_context_with_file() {
        let tmp = tempdir().unwrap();
        let workspace = tmp.path();
        let sub = workspace.join("subdir");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("AGENTS.md"), "# Subdir Instructions").unwrap();

        let result = discover_jit_context(&sub, workspace);
        assert!(result.is_some());
        let jit = result.unwrap();
        assert_eq!(jit.directory, sub);
        assert!(jit.context.has_instructions());
    }

    #[test]
    fn test_discover_jit_context_stops_at_workspace_root() {
        let tmp = tempdir().unwrap();
        let workspace = tmp.path();
        // Don't put AGENTS.md in workspace root
        let sub = workspace.join("subdir");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("AGENTS.md"), "# Subdir Instructions").unwrap();

        // Accessing workspace root should NOT find subdir's context
        let result = discover_jit_context(workspace, workspace);
        assert!(result.is_none());
    }

    #[test]
    fn test_discover_jit_context_walks_up() {
        let tmp = tempdir().unwrap();
        let workspace = tmp.path();
        let sub1 = workspace.join("a");
        let sub2 = sub1.join("b");
        fs::create_dir_all(&sub2).unwrap();

        // Put AGENTS.md in sub1, access sub2
        fs::write(sub1.join("AGENTS.md"), "# A Instructions").unwrap();

        let result = discover_jit_context(&sub2, workspace);
        assert!(result.is_some());
        let jit = result.unwrap();
        assert_eq!(jit.directory, sub1);
    }

    #[test]
    fn test_format_jit_context() {
        let tmp = tempdir().unwrap();
        let jit = JitContext {
            directory: tmp.path().join("mydir"),
            context: ProjectContext {
                instructions: Some("Test instructions".to_string()),
                source_path: Some(tmp.path().join("mydir/AGENTS.md")),
                warnings: Vec::new(),
                project_root: tmp.path().to_path_buf(),
                is_trusted: true,
            },
        };

        let formatted = format_jit_context(&jit);
        assert!(formatted.contains("JIT Context: mydir/AGENTS.md"));
        assert!(formatted.contains("Test instructions"));
        assert!(formatted.contains("End JIT Context"));
    }

    #[test]
    fn test_discover_jit_contexts_batch_dedup() {
        let tmp = tempdir().unwrap();
        let workspace = tmp.path();
        let sub = workspace.join("subdir");
        fs::create_dir(&sub).unwrap();
        fs::write(sub.join("AGENTS.md"), "# Sub Instructions").unwrap();

        let path1 = sub.join("file1.txt");
        let path2 = sub.join("file2.txt");
        fs::write(&path1, "").unwrap();
        fs::write(&path2, "").unwrap();

        let results = discover_jit_contexts_batch(
            &[path1.as_path(), path2.as_path()],
            workspace,
        );

        // Should only have one result (deduped)
        assert_eq!(results.len(), 1);
    }
}
