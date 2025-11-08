use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct User {
    pub seed: i32,
}

#[derive(Debug, Deserialize)]
pub struct QuestKeys {
    pub key1: String,
    #[serde(default)]
    pub key2: Option<String>,
    #[serde(default)]
    pub key3: Option<String>,
}

impl QuestKeys {
    pub fn get_key(&self, part: i32) -> Result<&str, String> {
        match part {
            1 => Ok(&self.key1),
            2 => self.key2.as_deref()
                .ok_or_else(|| "Part 2 key not available yet".to_string()),
            3 => self.key3.as_deref()
                .ok_or_else(|| "Part 3 key not available yet".to_string()),
            _ => Ok(&self.key1),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AnswerPayload {
    pub answer: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResponse {
    pub correct: bool,
    pub length_correct: bool,
    pub first_correct: bool,
    pub time: i64,
    pub global_place: i64,
    pub global_score: i64,
    #[serde(default)]
    pub message: String,
}
