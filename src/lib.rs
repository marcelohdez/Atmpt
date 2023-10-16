use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Ok};
use clap::Parser;
use directories_next::ProjectDirs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Atmpt {
    pub template: String,
}

impl Atmpt {
    pub fn parse_with(editor: &OsStr, dirs: &ProjectDirs) -> anyhow::Result<()> {
        let args = Self::parse();
        let template = &args.template;

        let templates = get_templates(dirs)?;
        let wanted_dir = get_dir(template, &templates)?;

        let tmp_dir = env::temp_dir().join(template);
        copy_dir_recursively(wanted_dir, &tmp_dir)?;

        std::env::set_current_dir(&tmp_dir).expect("Could not change to temp directory!");
        Command::new(editor)
            .arg(&tmp_dir)
            .spawn()?
            .wait()
            .expect("Could not launch editor!");

        Ok(())
    }
}

fn get_templates(dirs: &ProjectDirs) -> anyhow::Result<Vec<PathBuf>> {
    let mut templates = Vec::new();
    // should be ~/.local/share/atmpt/* on a linux system
    for entry in fs::read_dir(dirs.data_dir())? {
        let path = entry?.path();

        if path.is_dir() {
            templates.push(path);
        }
    }

    Ok(templates)
}

fn get_dir<'a>(name: &str, list: &'a [PathBuf]) -> anyhow::Result<&'a Path> {
    for path in list {
        if let Some(str) = path.file_name().and_then(|str| str.to_str()) {
            if str == name {
                return Ok(path);
            }
        }
    }

    bail!(format!(
        "No template '{name}' exists! Templates:\n{list:#?}"
    ))
}

// modified from https://stackoverflow.com/a/65192210/15425442
fn copy_dir_recursively(from: &Path, to: &Path) -> anyhow::Result<()> {
    fs::create_dir(to)?;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            copy_dir_recursively(&path, &to.join(entry.file_name()))?;
        } else {
            fs::copy(path, to.join(entry.file_name()))?;
        }
    }

    Ok(())
}
