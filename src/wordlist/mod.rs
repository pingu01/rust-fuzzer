use std::{fs, path::PathBuf};

use anyhow::{Ok, bail};

pub struct Wordlist {
    file_contents: String,
    pub entries: Vec<String>,
}

impl Wordlist {
    pub fn new(file_path: PathBuf) -> anyhow::Result<Self> {
        if !file_path.exists() {
            bail!("this file does not exist nigga")
        }

        let file_contents = fs::read_to_string(&file_path)?;

        let entries = file_contents
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Ok(Self {
            file_contents,
            entries,
        })
    }
}
