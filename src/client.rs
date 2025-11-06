use log::{debug, info};
use reqwest::{Client, StatusCode};
use std::env;
use std::fs;

use crate::crypto::decrypt_aes_cbc;
use crate::error::{EcError, Result};
use crate::models::{AnswerPayload, QuestKeys, SubmitResponse, User};

const BASE_URL: &str = "https://everybody.codes";
const CDN_URL: &str = "https://everybody-codes.b-cdn.net";
const USER_AGENT: &str = "ec-cli/0.1.0";

pub struct EcClient {
    client: Client,
    cookie: String,
    user_seed: Option<i32>,
}

impl EcClient {
    /// Create a new EC client with authentication cookie
    pub fn new() -> Result<Self> {
        let cookie = Self::load_cookie()?;

        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()?;

        Ok(Self {
            client,
            cookie,
            user_seed: None,
        })
    }

    /// Format cookie for HTTP header
    fn cookie_header(&self) -> String {
        format!("everybody-codes={}", &self.cookie)
    }

    /// Load cookie from environment variable or file
    fn load_cookie() -> Result<String> {
        // Try environment variable first
        if let Ok(cookie) = env::var("EC_COOKIE") {
            debug!("Loaded cookie from EC_COOKIE environment variable");
            return Ok(cookie);
        }

        // Try ~/.everybodycodes.cookie file
        if let Some(home_dir) = dirs::home_dir() {
            let cookie_path = home_dir.join(".everybodycodes.cookie");
            if cookie_path.exists() {
                debug!("Loading cookie from {:?}", cookie_path);
                let cookie = fs::read_to_string(cookie_path)?
                    .trim()
                    .to_string();
                return Ok(cookie);
            }
        }

        // Try config directory
        if let Some(config_dir) = dirs::config_dir() {
            let cookie_path = config_dir.join("everybodycodes").join("cookie");
            if cookie_path.exists() {
                debug!("Loading cookie from {:?}", cookie_path);
                let cookie = fs::read_to_string(cookie_path)?
                    .trim()
                    .to_string();
                return Ok(cookie);
            }
        }

        Err(EcError::MissingCookie)
    }

    /// Fetch user seed from API (cached after first call)
    pub async fn get_user_seed(&mut self) -> Result<i32> {
        if let Some(seed) = self.user_seed {
            return Ok(seed);
        }

        info!("Fetching user seed...");
        let url = format!("{}/api/user/me", BASE_URL);

        let response = self.client
            .get(&url)
            .header("Cookie", &self.cookie_header())
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EcError::HttpError {
                status: response.status().as_u16(),
                message: format!("Failed to fetch user seed: {}", response.status()),
            });
        }

        let body = response.text().await?;
        debug!("User API response: {}", body);

        let user: User = serde_json::from_str(&body)?;
        self.user_seed = Some(user.seed);
        debug!("User seed: {}", user.seed);

        Ok(user.seed)
    }

    /// Fetch quest keys (key1, key2, key3) for decryption
    pub async fn fetch_quest_keys(&self, year: i32, day: i32) -> Result<QuestKeys> {
        info!("Fetching quest keys for {}/{}...", year, day);
        let url = format!("{}/api/event/{}/quest/{}", BASE_URL, year, day);

        let response = self.client
            .get(&url)
            .header("Cookie", &self.cookie_header())
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(EcError::HttpError {
                status: status.as_u16(),
                message: format!("Failed to fetch quest keys: {}", status),
            });
        }

        // Get response text first for better error messages
        let body = response.text().await?;
        debug!("Quest keys response: {}", body);

        let keys: QuestKeys = serde_json::from_str(&body)
            .map_err(|e| EcError::JsonError(e))?;
        debug!("Fetched quest keys for {}/{}", year, day);

        Ok(keys)
    }

    /// Fetch and decrypt puzzle input
    pub async fn fetch_input(&mut self, year: i32, day: i32, part: i32) -> Result<String> {
        let seed = self.get_user_seed().await?;
        let keys = self.fetch_quest_keys(year, day).await?;
        let key = keys.get_key(part)
            .map_err(|e| EcError::DecryptionError(e))?;

        info!("Downloading encrypted input for {}/{} part {}...", year, day, part);
        let url = format!("{}/assets/{}/{}/input/{}.json", CDN_URL, year, day, seed);
        debug!("Fetching input from URL: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EcError::HttpError {
                status: response.status().as_u16(),
                message: format!("Failed to fetch input: {}", response.status()),
            });
        }

        let body = response.text().await?;
        debug!("Input response (first 100 chars): {}", &body.chars().take(100).collect::<String>());

        info!("Decrypting input...");

        // Parse as JSON to get the encrypted string
        let encrypted = if body.starts_with('{') {
            // New format: JSON object
            let encrypted_parts: serde_json::Value = serde_json::from_str(&body)?;
            encrypted_parts[&part.to_string()]
                .as_str()
                .ok_or_else(|| EcError::DecryptionError(format!("Missing part {} in input", part)))?
                .to_string()
        } else {
            // Old format: plain string (with quotes)
            body.trim_matches('"').to_string()
        };

        let decrypted = decrypt_aes_cbc(&encrypted, key)?;

        Ok(decrypted)
    }

    /// Fetch and decrypt puzzle description
    pub async fn fetch_description(&self, year: i32, day: i32) -> Result<String> {
        let keys = self.fetch_quest_keys(year, day).await?;

        info!("Downloading encrypted description for {}/{}...", year, day);
        let url = format!("{}/assets/{}/{}/description.json", CDN_URL, year, day);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EcError::HttpError {
                status: response.status().as_u16(),
                message: format!("Failed to fetch description: {}", response.status()),
            });
        }

        let body = response.text().await?;
        debug!("Encrypted description (first 100 chars): {}", &body.chars().take(100).collect::<String>());

        info!("Decrypting description...");

        // Parse as JSON object with parts "1", "2", "3"
        let encrypted_parts: serde_json::Value = serde_json::from_str(&body)?;

        // Decrypt all available parts and combine them
        let mut combined = String::new();

        for (part_num, key_opt) in [
            (1, Some(&keys.key1)),
            (2, keys.key2.as_ref()),
            (3, keys.key3.as_ref()),
        ] {
            if let Some(key) = key_opt {
                if let Some(encrypted) = encrypted_parts[&part_num.to_string()].as_str() {
                    debug!("Decrypting description part {}...", part_num);
                    let decrypted = decrypt_aes_cbc(encrypted, key)?;

                    if !combined.is_empty() {
                        combined.push_str("\n\n");
                        combined.push_str(&"=".repeat(80));
                        combined.push_str(&format!("\n PART {} \n", part_num));
                        combined.push_str(&"=".repeat(80));
                        combined.push_str("\n\n");
                    }
                    combined.push_str(&decrypted);
                }
            }
        }

        Ok(combined)
    }

    /// Submit an answer for a puzzle
    pub async fn submit_answer(
        &self,
        year: i32,
        day: i32,
        part: i32,
        answer: &str,
    ) -> Result<SubmitResponse> {
        info!("Submitting answer for {}/{} part {}...", year, day, part);
        let url = format!(
            "{}/api/event/{}/quest/{}/part/{}/answer",
            BASE_URL, year, day, part
        );

        let payload = AnswerPayload {
            answer: answer.to_string(),
        };

        let response = self.client
            .post(&url)
            .header("Cookie", &self.cookie_header())
            .json(&payload)
            .send()
            .await?;

        match response.status() {
            StatusCode::CONFLICT => {
                return Err(EcError::AlreadySubmitted);
            }
            status if !status.is_success() => {
                return Err(EcError::HttpError {
                    status: status.as_u16(),
                    message: format!("Failed to submit answer: {}", status),
                });
            }
            _ => {}
        }

        let submit_response: SubmitResponse = response.json().await?;

        Ok(submit_response)
    }
}
