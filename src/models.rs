use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct User {
    pub seed: i32,
}

#[derive(Debug, Deserialize)]
pub struct QuestKeys {
    pub key1: String,
    pub key2: String,
    pub key3: String,
}

impl QuestKeys {
    pub fn get_key(&self, part: i32) -> &str {
        match part {
            1 => &self.key1,
            2 => &self.key2,
            3 => &self.key3,
            _ => &self.key1,
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
    pub local_time: i64,
    pub global_time: i64,
    pub global_place: i64,
    pub global_score: i64,
    #[serde(default)]
    pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub struct Quest {
    pub year: i32,
    pub day: i32,
    pub part: i32,
}

impl Quest {
    pub fn new(year: i32, day: i32, part: i32) -> Self {
        Self { year, day, part }
    }
}
