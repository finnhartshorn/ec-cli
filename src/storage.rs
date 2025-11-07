use log::{debug, info};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Result;

pub struct Storage {
    base_path: PathBuf,
    description_path: Option<PathBuf>,
    input_path: Option<PathBuf>,
}

impl Storage {
    /// Create a new storage manager with base path (defaults to "data")
    pub fn new(base_path: Option<PathBuf>) -> Self {
        let base_path = base_path.unwrap_or_else(|| PathBuf::from("data"));
        Self {
            base_path,
            description_path: None,
            input_path: None,
        }
    }

    /// Create storage manager with custom description path
    pub fn with_description_path(mut self, path: PathBuf) -> Self {
        self.description_path = Some(path);
        self
    }

    /// Create storage manager with custom input path
    pub fn with_input_path(mut self, path: PathBuf) -> Self {
        self.input_path = Some(path);
        self
    }

    /// Get the path for inputs directory
    fn inputs_dir(&self, year: i32) -> PathBuf {
        self.base_path.join(year.to_string()).join("inputs")
    }

    /// Get the path for samples directory
    fn samples_dir(&self, year: i32) -> PathBuf {
        self.base_path.join(year.to_string()).join("samples")
    }

    /// Get the path for descriptions directory
    fn descriptions_dir(&self, year: i32) -> PathBuf {
        self.base_path.join(year.to_string()).join("descriptions")
    }

    /// Ensure directory exists
    fn ensure_dir<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        if !path.exists() {
            debug!("Creating directory: {:?}", path);
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// Save puzzle input to file
    pub fn save_input(&self, year: i32, day: i32, part: i32, content: &str) -> Result<PathBuf> {
        let path = if let Some(custom_path) = &self.input_path {
            // Use custom path directly
            custom_path.clone()
        } else {
            // Use default path structure
            let dir = self.inputs_dir(year);
            Self::ensure_dir(&dir)?;
            let filename = format!("{}-{}.txt", day, part);
            dir.join(filename)
        };

        // Ensure parent directory exists for custom paths
        if let Some(parent) = path.parent() {
            Self::ensure_dir(parent)?;
        }

        info!("Saving input to {:?}", path);
        fs::write(&path, content)?;

        Ok(path)
    }

    /// Save sample/example data to file
    pub fn save_sample(&self, year: i32, day: i32, part: i32, content: &str) -> Result<PathBuf> {
        let dir = self.samples_dir(year);
        Self::ensure_dir(&dir)?;

        let filename = format!("{}-{}.txt", day, part);
        let path = dir.join(filename);

        info!("Saving sample to {:?}", path);
        fs::write(&path, content)?;

        Ok(path)
    }

    /// Save puzzle description to file
    pub fn save_description(&self, year: i32, day: i32, content: &str) -> Result<PathBuf> {
        let path = if let Some(custom_path) = &self.description_path {
            // Use custom path directly
            custom_path.clone()
        } else {
            // Use default path structure
            let dir = self.descriptions_dir(year);
            Self::ensure_dir(&dir)?;
            let filename = format!("{}.html", day);
            dir.join(filename)
        };

        // Ensure parent directory exists for custom paths
        if let Some(parent) = path.parent() {
            Self::ensure_dir(parent)?;
        }

        info!("Saving description to {:?}", path);
        fs::write(&path, content)?;

        Ok(path)
    }

    /// Load puzzle input from file
    pub fn load_input(&self, year: i32, day: i32, part: i32) -> Result<String> {
        let dir = self.inputs_dir(year);
        let filename = format!("{}-{}.txt", day, part);
        let path = dir.join(filename);

        debug!("Loading input from {:?}", path);
        let content = fs::read_to_string(&path)?;

        Ok(content)
    }

    /// Load puzzle description from file
    pub fn load_description(&self, year: i32, day: i32) -> Result<String> {
        let dir = self.descriptions_dir(year);
        let filename = format!("{}.html", day);
        let path = dir.join(filename);

        debug!("Loading description from {:?}", path);
        let content = fs::read_to_string(&path)?;

        Ok(content)
    }

    /// Check if input file exists
    pub fn has_input(&self, year: i32, day: i32, part: i32) -> bool {
        let dir = self.inputs_dir(year);
        let filename = format!("{}-{}.txt", day, part);
        let path = dir.join(filename);
        path.exists()
    }

    /// Check if description file exists
    pub fn has_description(&self, year: i32, day: i32) -> bool {
        let dir = self.descriptions_dir(year);
        let filename = format!("{}.html", day);
        let path = dir.join(filename);
        path.exists()
    }
}
