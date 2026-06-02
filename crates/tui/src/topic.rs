//! Topic management for sessions.
//!
//! Topics provide a way to organize conversation context into named units,
//! making it easier to track what the agent is working on across long sessions.
//!
//! Unlike flat session histories, topics let the agent:
//! - Name the current work unit
//! - Switch between topics within a session
//! - Persist topic metadata for later reference
//! - Track strategic intent per topic

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Maximum number of topics per session
const MAX_TOPICS_PER_SESSION: usize = 50;

/// Maximum length for topic title
const MAX_TOPIC_TITLE_LEN: usize = 200;

/// Maximum length for topic summary
const MAX_TOPIC_SUMMARY_LEN: usize = 2000;

/// Maximum length for strategic intent
const MAX_TOPIC_INTENT_LEN: usize = 1000;

/// A topic represents a named unit of work within a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    /// Unique identifier for this topic
    pub id: String,
    /// Human-readable title
    pub title: String,
    /// Optional summary of what this topic covers
    pub summary: Option<String>,
    /// Strategic intent or goal for this topic
    pub strategic_intent: Option<String>,
    /// Timestamp when this topic was created (Unix seconds)
    pub created_at: u64,
    /// Timestamp when this topic was last updated (Unix seconds)
    pub updated_at: u64,
    /// Number of messages in this topic
    pub message_count: usize,
    /// Whether this topic is the currently active one
    pub is_active: bool,
}

impl Topic {
    /// Create a new topic
    pub fn new(title: String) -> Self {
        let now = Self::now_secs();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            summary: None,
            strategic_intent: None,
            created_at: now,
            updated_at: now,
            message_count: 0,
            is_active: true,
        }
    }

    /// Create a new topic with summary and intent
    pub fn with_details(
        title: String,
        summary: Option<String>,
        strategic_intent: Option<String>,
    ) -> Self {
        let now = Self::now_secs();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            summary,
            strategic_intent,
            created_at: now,
            updated_at: now,
            message_count: 0,
            is_active: true,
        }
    }

    /// Get current Unix timestamp in seconds
    fn now_secs() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Validate topic title
    pub fn validate_title(title: &str) -> Result<(), String> {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            return Err("Topic title cannot be empty".to_string());
        }
        if trimmed.len() > MAX_TOPIC_TITLE_LEN {
            return Err(format!(
                "Topic title too long ({} chars, max {})",
                trimmed.len(),
                MAX_TOPIC_TITLE_LEN
            ));
        }
        Ok(())
    }

    /// Validate topic summary
    pub fn validate_summary(summary: &str) -> Result<(), String> {
        if summary.len() > MAX_TOPIC_SUMMARY_LEN {
            return Err(format!(
                "Topic summary too long ({} chars, max {})",
                summary.len(),
                MAX_TOPIC_SUMMARY_LEN
            ));
        }
        Ok(())
    }

    /// Validate strategic intent
    pub fn validate_intent(intent: &str) -> Result<(), String> {
        if intent.len() > MAX_TOPIC_INTENT_LEN {
            return Err(format!(
                "Strategic intent too long ({} chars, max {})",
                intent.len(),
                MAX_TOPIC_INTENT_LEN
            ));
        }
        Ok(())
    }
}

/// Topic manager for a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicManager {
    /// All topics in this session
    topics: Vec<Topic>,
    /// Path to the topic persistence file
    persistence_path: Option<PathBuf>,
}

impl TopicManager {
    /// Create a new empty topic manager
    pub fn new() -> Self {
        Self {
            topics: Vec::new(),
            persistence_path: None,
        }
    }

    /// Create a topic manager with persistence
    pub fn with_persistence(path: PathBuf) -> Self {
        Self {
            topics: Vec::new(),
            persistence_path: Some(path),
        }
    }

    /// Load topic manager from a file
    pub fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::with_persistence(path.to_path_buf()));
        }

        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read topic file: {e}"))?;

        let manager: Self = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse topic file: {e}"))?;

        Ok(manager)
    }

    /// Save topic manager to file
    pub fn save(&self) -> Result<(), String> {
        let Some(path) = &self.persistence_path else {
            return Ok(());
        };

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create topic directory: {e}"))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize topics: {e}"))?;

        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write topic file: {e}"))?;

        Ok(())
    }

    /// Get the currently active topic
    pub fn active_topic(&self) -> Option<&Topic> {
        self.topics.iter().find(|t| t.is_active)
    }

    /// Get all topics
    pub fn topics(&self) -> &[Topic] {
        &self.topics
    }

    /// Get topic count
    pub fn topic_count(&self) -> usize {
        self.topics.len()
    }

    /// Create a new topic and make it active
    pub fn create_topic(
        &mut self,
        title: String,
        summary: Option<String>,
        strategic_intent: Option<String>,
    ) -> Result<&Topic, String> {
        Topic::validate_title(&title)?;
        if let Some(ref s) = summary {
            Topic::validate_summary(s)?;
        }
        if let Some(ref i) = strategic_intent {
            Topic::validate_intent(i)?;
        }

        if self.topics.len() >= MAX_TOPICS_PER_SESSION {
            return Err(format!(
                "Maximum topics per session reached ({MAX_TOPICS_PER_SESSION})"
            ));
        }

        // Deactivate current topic
        for topic in &mut self.topics {
            topic.is_active = false;
        }

        let topic = Topic::with_details(title, summary, strategic_intent);
        self.topics.push(topic);

        let _ = self.save();
        Ok(self.topics.last().unwrap())
    }

    /// Switch to an existing topic by ID or title substring
    pub fn switch_topic(&mut self, query: &str) -> Result<&Topic, String> {
        let query_lower = query.to_lowercase();

        // Try exact ID match first
        if let Some(idx) = self.topics.iter().position(|t| t.id == query) {
            for topic in &mut self.topics {
                topic.is_active = false;
            }
            self.topics[idx].is_active = true;
            let _ = self.save();
            return Ok(&self.topics[idx]);
        }

        // Try case-insensitive title contains
        if let Some(idx) = self
            .topics
            .iter()
            .position(|t| t.title.to_lowercase().contains(&query_lower))
        {
            for topic in &mut self.topics {
                topic.is_active = false;
            }
            self.topics[idx].is_active = true;
            let _ = self.save();
            return Ok(&self.topics[idx]);
        }

        Err(format!("No topic found matching '{query}'"))
    }

    /// Update the active topic's metadata
    pub fn update_active_topic(
        &mut self,
        title: Option<String>,
        summary: Option<String>,
        strategic_intent: Option<String>,
    ) -> Result<&Topic, String> {
        let idx = self
            .topics
            .iter()
            .position(|t| t.is_active)
            .ok_or_else(|| "No active topic".to_string())?;

        if let Some(t) = title {
            Topic::validate_title(&t)?;
            self.topics[idx].title = t;
        }
        if let Some(s) = summary {
            Topic::validate_summary(&s)?;
            self.topics[idx].summary = Some(s);
        }
        if let Some(i) = strategic_intent {
            Topic::validate_intent(&i)?;
            self.topics[idx].strategic_intent = Some(i);
        }

        self.topics[idx].updated_at = Topic::now_secs();
        let _ = self.save();
        Ok(&self.topics[idx])
    }

    /// Increment message count for the active topic
    pub fn increment_message_count(&mut self) {
        if let Some(topic) = self.topics.iter_mut().find(|t| t.is_active) {
            topic.message_count += 1;
            topic.updated_at = Topic::now_secs();
            let _ = self.save();
        }
    }

    /// Delete a topic by ID
    pub fn delete_topic(&mut self, id: &str) -> Result<(), String> {
        let was_active = self
            .topics
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.is_active)
            .unwrap_or(false);

        self.topics.retain(|t| t.id != id);

        // If we deleted the active topic, activate the most recent one
        if was_active {
            if let Some(last) = self.topics.last_mut() {
                last.is_active = true;
            }
        }

        let _ = self.save();
        Ok(())
    }

    /// Format topic list for display
    pub fn format_topic_list(&self) -> String {
        if self.topics.is_empty() {
            return "No topics in this session.".to_string();
        }

        let mut lines = Vec::new();
        lines.push(format!("Topics ({}):", self.topics.len()));
        lines.push(String::new());

        for topic in self.topics.iter() {
            let marker = if topic.is_active { "▶" } else { " " };
            let intent_label = topic
                .strategic_intent
                .as_ref()
                .map(|i| format!(" | {}", truncate_str(i, 60)))
                .unwrap_or_default();
            lines.push(format!(
                "{} {}{} [{} msgs]{}",
                marker,
                topic.title,
                topic
                    .id
                    .chars()
                    .take(8)
                    .collect::<String>(),
                topic.message_count,
                intent_label,
            ));
        }

        lines.join("\n")
    }

    /// Get the active topic's strategic intent for system prompt injection
    pub fn active_intent_block(&self) -> Option<String> {
        let topic = self.active_topic()?;
        let title = &topic.title;
        let intent = topic.strategic_intent.as_deref().unwrap_or("...");
        let summary = topic.summary.as_deref().unwrap_or("...");

        Some(format!(
            "<current_topic>\n\
             title: {title}\n\
             summary: {summary}\n\
             strategic_intent: {intent}\n\
             messages: {}\n\
             </current_topic>",
            topic.message_count,
        ))
    }
}

impl Default for TopicManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Truncate a string to max chars with ellipsis
fn truncate_str(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(max).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_topic() {
        let mut mgr = TopicManager::new();
        let topic = mgr
            .create_topic(
                "Fix bug".to_string(),
                Some("Fix the login bug".to_string()),
                None,
            )
            .unwrap();

        assert_eq!(topic.title, "Fix bug");
        assert!(topic.is_active);
        assert_eq!(mgr.topic_count(), 1);
    }

    #[test]
    fn test_switch_topic() {
        let mut mgr = TopicManager::new();
        mgr.create_topic("Topic A".to_string(), None, None)
            .unwrap();
        mgr.create_topic("Topic B".to_string(), None, None)
            .unwrap();

        assert_eq!(mgr.active_topic().unwrap().title, "Topic B");

        mgr.switch_topic("Topic A").unwrap();
        assert_eq!(mgr.active_topic().unwrap().title, "Topic B");

        // Deactivate all first
        for t in &mut mgr.topics {
            t.is_active = false;
        }
        mgr.topics[0].is_active = true;

        assert_eq!(mgr.active_topic().unwrap().title, "Topic A");
    }

    #[test]
    fn test_persistence() {
        let tmp = tempdir().unwrap();
        let path = tmp.path().join("topics.json");

        {
            let mut mgr = TopicManager::with_persistence(path.clone());
            mgr.create_topic("Persistent Topic".to_string(), None, None)
                .unwrap();
        }

        let loaded = TopicManager::load(&path).unwrap();
        assert_eq!(loaded.topic_count(), 1);
        assert_eq!(loaded.topics[0].title, "Persistent Topic");
    }

    #[test]
    fn test_max_topics() {
        let mut mgr = TopicManager::new();
        for i in 0..50 {
            mgr.create_topic(format!("Topic {i}"), None, None).unwrap();
        }
        assert!(mgr
            .create_topic("One more".to_string(), None, None)
            .is_err());
    }

    #[test]
    fn test_intent_block() {
        let mut mgr = TopicManager::new();
        mgr.create_topic(
            "My Topic".to_string(),
            Some("Summary here".to_string()),
            Some("Strategic intent here".to_string()),
        )
        .unwrap();

        let block = mgr.active_intent_block().unwrap();
        assert!(block.contains("My Topic"));
        assert!(block.contains("Summary here"));
        assert!(block.contains("Strategic intent here"));
    }
}
