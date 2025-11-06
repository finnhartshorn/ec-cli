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
            .header("Cookie", &self.cookie)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EcError::HttpError {
                status: response.status().as_u16(),
                message: format!("Failed to fetch user seed: {}", response.status()),
            });
        }

        let user: User = response.json().await?;
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
            .header("Cookie", &self.cookie)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(EcError::HttpError {
                status: response.status().as_u16(),
                message: format!("Failed to fetch quest keys: {}", response.status()),
            });
        }

        let keys: QuestKeys = response.json().await?;
        debug!("Fetched quest keys for {}/{}", year, day);

        Ok(keys)
    }

    /// Fetch and decrypt puzzle input
    pub async fn fetch_input(&mut self, year: i32, day: i32, part: i32) -> Result<String> {
        let seed = self.get_user_seed().await?;
        let keys = self.fetch_quest_keys(year, day).await?;
        let key = keys.get_key(part);

        info!("Downloading encrypted input for {}/{} part {}...", year, day, part);
        let url = format!("{}/assets/{}/{}/input/{}.json", CDN_URL, year, day, seed);

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

        let encrypted = response.text().await?;

        info!("Decrypting input...");
        let decrypted = decrypt_aes_cbc(&encrypted.trim_matches('"'), key)?;

        Ok(decrypted)
    }

    /// Fetch and decrypt puzzle description
    pub async fn fetch_description(&self, year: i32, day: i32) -> Result<String> {
        let keys = self.fetch_quest_keys(year, day).await?;
        let key = &keys.key1; // Description uses key1

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

        let encrypted = response.text().await?;

        info!("Decrypting description...");
        let decrypted = decrypt_aes_cbc(&encrypted.trim_matches('"'), key)?;

        Ok(decrypted)
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
            .header("Cookie", &self.cookie)
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
