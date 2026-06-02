//! Ask User tool - allows AI to向用户提问，支持选择题和多选题

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::tools::spec::{
    ApprovalRequirement, ToolCapability, ToolContext, ToolError, ToolResult, ToolSpec,
};

/// Types of questions that can be asked
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    /// Free text input
    Text,
    /// Single choice from options
    Choice,
    /// Multiple choices from options
    MultiChoice,
}

/// A single option for choice/multi_choice questions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    /// Display label for the option
    pub label: String,
    /// Optional description of the option
    pub description: Option<String>,
}

/// A question to ask the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    /// The question text
    pub question: String,
    /// Type of question (text, choice, multi_choice)
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    /// Available options (required for choice/multi_choice)
    #[serde(default)]
    pub options: Vec<QuestionOption>,
    /// Whether multiple selections are allowed (for multi_choice)
    #[serde(default)]
    pub multi_select: bool,
}

/// Input for the ask_user tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskUserInput {
    /// List of questions to ask
    pub questions: Vec<Question>,
}

/// Response from the user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    /// Index of the selected option(s), or text input
    pub answer: UserAnswer,
}

/// User answer - either text or option indices
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserAnswer {
    /// Text input
    Text(String),
    /// Selected option index (single choice)
    Single(usize),
    /// Selected option indices (multiple choice)
    Multiple(Vec<usize>),
}

/// Tool for asking the user questions
pub struct AskUserTool;

impl AskUserTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AskUserTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolSpec for AskUserTool {
    fn name(&self) -> &'static str {
        "ask_user"
    }

    fn description(&self) -> &'static str {
        "Ask the user a question to gather information or make decisions. Supports free text input, single choice, and multiple choice questions. Use this when you need user input to proceed with a task."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "questions": {
                    "type": "array",
                    "description": "List of questions to ask the user",
                    "items": {
                        "type": "object",
                        "properties": {
                            "question": {
                                "type": "string",
                                "description": "The question text"
                            },
                            "type": {
                                "type": "string",
                                "enum": ["text", "choice", "multi_choice"],
                                "description": "Type of question"
                            },
                            "options": {
                                "type": "array",
                                "description": "Available options (required for choice/multi_choice)",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "label": {
                                            "type": "string",
                                            "description": "Display label for the option"
                                        },
                                        "description": {
                                            "type": "string",
                                            "description": "Optional description of the option"
                                        }
                                    },
                                    "required": ["label"]
                                }
                            },
                            "multi_select": {
                                "type": "boolean",
                                "description": "Whether multiple selections are allowed (for multi_choice)",
                                "default": false
                            }
                        },
                        "required": ["question", "type"]
                    }
                }
            },
            "required": ["questions"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![] // No special capabilities needed
    }

    fn approval_requirement(&self) -> ApprovalRequirement {
        ApprovalRequirement::Auto // Auto-approve, the tool itself handles user interaction
    }

    async fn execute(
        &self,
        input: serde_json::Value,
        _context: &ToolContext,
    ) -> Result<ToolResult, ToolError> {
        let questions: Vec<Question> = serde_json::from_value(
            input
                .get("questions")
                .cloned()
                .ok_or_else(|| ToolError::invalid_input("Missing 'questions' array"))?,
        )
        .map_err(|e| ToolError::invalid_input(format!("Invalid questions format: {e}")))?;

        if questions.is_empty() {
            return Err(ToolError::invalid_input(
                "At least one question is required",
            ));
        }

        // Validate questions
        for (i, q) in questions.iter().enumerate() {
            if q.question.trim().is_empty() {
                return Err(ToolError::invalid_input(format!(
                    "Question {} has empty question text",
                    i + 1
                )));
            }

            match q.question_type {
                QuestionType::Choice => {
                    if q.options.len() < 2 {
                        return Err(ToolError::invalid_input(format!(
                            "Question {} (choice) requires at least 2 options",
                            i + 1
                        )));
                    }
                }
                QuestionType::MultiChoice => {
                    if q.options.len() < 2 {
                        return Err(ToolError::invalid_input(format!(
                            "Question {} (multi_choice) requires at least 2 options",
                            i + 1
                        )));
                    }
                }
                QuestionType::Text => {} // No validation needed
            }
        }

        // Format questions for display
        let mut display = String::from("Questions for the user:\n\n");

        for (i, q) in questions.iter().enumerate() {
            display.push_str(&format!("{}. {}\n", i + 1, q.question));

            match q.question_type {
                QuestionType::Text => {
                    display.push_str("   (Please provide a text response)\n");
                }
                QuestionType::Choice => {
                    display.push_str("   Choose one:\n");
                    for (j, opt) in q.options.iter().enumerate() {
                        display.push_str(&format!("   {} - {}", j + 1, opt.label));
                        if let Some(desc) = &opt.description {
                            display.push_str(&format!(": {desc}"));
                        }
                        display.push('\n');
                    }
                }
                QuestionType::MultiChoice => {
                    display.push_str("   Choose one or more:\n");
                    for (j, opt) in q.options.iter().enumerate() {
                        display.push_str(&format!("   {} - {}", j + 1, opt.label));
                        if let Some(desc) = &opt.description {
                            display.push_str(&format!(": {desc}"));
                        }
                        display.push('\n');
                    }
                }
            }
            display.push('\n');
        }

        // Store questions in context for the UI to render
        // The actual user interaction happens through the UI layer
        // For now, return the formatted questions
        Ok(ToolResult::success(format!(
            "{display}\n\
             Waiting for user response..."
        )))
    }
}

/// Format user responses for display
pub fn format_responses(questions: &[Question], responses: &[UserResponse]) -> String {
    let mut output = String::from("User responses:\n\n");

    for (q, r) in questions.iter().zip(responses.iter()) {
        output.push_str(&format!("Q: {}\n", q.question));

        match &r.answer {
            UserAnswer::Text(text) => {
                output.push_str(&format!("A: {text}\n"));
            }
            UserAnswer::Single(idx) => {
                if let Some(opt) = q.options.get(*idx) {
                    output.push_str(&format!("A: {}\n", opt.label));
                }
            }
            UserAnswer::Multiple(indices) => {
                let labels: Vec<&str> = indices
                    .iter()
                    .filter_map(|idx| q.options.get(*idx).map(|o| o.label.as_str()))
                    .collect();
                output.push_str(&format!("A: {}\n", labels.join(", ")));
            }
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question_validation() {
        let q = Question {
            question: "".to_string(),
            question_type: QuestionType::Text,
            options: vec![],
            multi_select: false,
        };

        assert!(q.question.trim().is_empty());
    }

    #[test]
    fn test_format_responses() {
        let questions = vec![
            Question {
                question: "What is your name?".to_string(),
                question_type: QuestionType::Text,
                options: vec![],
                multi_select: false,
            },
            Question {
                question: "Choose a color".to_string(),
                question_type: QuestionType::Choice,
                options: vec![
                    QuestionOption {
                        label: "Red".to_string(),
                        description: None,
                    },
                    QuestionOption {
                        label: "Blue".to_string(),
                        description: None,
                    },
                ],
                multi_select: false,
            },
        ];

        let responses = vec![
            UserResponse {
                answer: UserAnswer::Text("Alice".to_string()),
            },
            UserResponse {
                answer: UserAnswer::Single(0),
            },
        ];

        let output = format_responses(&questions, &responses);
        assert!(output.contains("Alice"));
        assert!(output.contains("Red"));
    }
}
