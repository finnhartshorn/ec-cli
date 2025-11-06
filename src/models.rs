use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct User {
    pub seed: i32,
}

#[derive(Debug, Deserialize)]
pub struct EncryptedContent {
    #[serde(rename = "1")]
    pub part1: Option<String>,
    #[serde(rename = "2")]
    pub part2: Option<String>,
    #[serde(rename = "3")]
    pub part3: Option<String>,
}

impl EncryptedContent {
    pub fn get_part(&self, part: i32) -> Result<&str, String> {
        match part {
            1 => self.part1.as_deref()
                .ok_or_else(|| "Part 1 not available".to_string()),
            2 => self.part2.as_deref()
                .ok_or_else(|| "Part 2 not available".to_string()),
            3 => self.part3.as_deref()
                .ok_or_else(|| "Part 3 not available".to_string()),
            _ => Err(format!("Invalid part: {}", part)),
        }
    }
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
