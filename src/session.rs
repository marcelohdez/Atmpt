use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub last_template: String,
    pub previous_attempt: PathBuf,
}

impl Session {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let file = File::open(path)
            .context("Could not open session file, have you run atmpt recently?")?;
        let session: Self = serde_json::from_reader(BufReader::new(file))
            .context("Failed to read session file!")?;

        Ok(session)
    }
}
