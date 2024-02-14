use std::{
    fmt::Display,
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context};

pub struct Templates(Vec<PathBuf>);

impl Deref for Templates {
    type Target = Vec<PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Templates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let templates: Vec<_> = self
            .iter()
            .flat_map(|path| path.file_name())
            .map(|name| name.to_string_lossy())
            .collect();

        write!(f, "{}", templates.join("  "))
    }
}

impl TryFrom<&Path> for Templates {
    type Error = anyhow::Error;

    fn try_from(data_dir: &Path) -> Result<Self, Self::Error> {
        if !data_dir.exists() {
            bail!("Template directory does not exist:\n  {data_dir:?}\nCreate it along with some templates inside!");
        }

        let entries =
            fs::read_dir(data_dir).with_context(|| format!("Failed to read {data_dir:?}"))?;

        let mut templates = Vec::new();
        for entry in entries {
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

        Err(anyhow!(
            "No template '{name}' exists! Available templates:\n{self}"
        ))
    }
}
