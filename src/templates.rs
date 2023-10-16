use std::{
    fmt::Display,
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::bail;

pub struct Templates(Vec<PathBuf>);

impl Deref for Templates {
    type Target = Vec<PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Templates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for path in &self.0 {
            let Some(file_name) = path.file_name().and_then(|s| s.to_str()) else {
                return Err(std::fmt::Error);
            };

            write!(f, "  {}", file_name)?;
        }

        Ok(())
    }
}

impl TryFrom<&Path> for Templates {
    type Error = std::io::Error;

    fn try_from(data_dir: &Path) -> Result<Self, Self::Error> {
        let mut templates = Vec::new();
        // should be ~/.local/share/atmpt/* on a linux system
        for entry in fs::read_dir(data_dir)? {
            let path = entry?.path();

            if path.is_dir() {
                templates.push(path);
            }
        }

        Ok(Templates(templates))
    }
}

impl Templates {
    pub fn find<'a>(&'a self, name: &str) -> anyhow::Result<&'a Path> {
        for path in self.deref() {
            if let Some(str) = path.file_name().and_then(|str| str.to_str()) {
                if str == name {
                    return Ok(path);
                }
            }
        }

        bail!(format!(
            "No template '{name}' exists! Available templates:\n{self}"
        ))
    }
}
