use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{anyhow, Ok};
use atmpt::Atmpt;
use clap::Parser;
use directories_next::ProjectDirs;

fn main() -> anyhow::Result<()> {
    let editor_env_pair = std::env::vars_os().find(|(k, _)| {
        if let Some(str) = k.to_str() {
            return str == "EDITOR";
        }
        false
    });

    let Some((_, editor)) = editor_env_pair else {
        return Err(anyhow!("$EDITOR environment variable is not set!"));
    };
    let Some(editor) = editor.to_str() else {
        return Err(anyhow!(
            "$EDITOR environment variable is not valid Unicode?"
        ));
    };

    let args = Atmpt::parse();
    let Some(dirs) = ProjectDirs::from("me", "marcelohdez", "Atmpt") else {
        return Err(anyhow!("Could not get project dirs!"));
    };

    let mut wanted_template = None;
    let templates = match get_templates(&dirs) {
        anyhow::Result::Ok(v) => v,
        Err(_) => Vec::new(),
    };

    for path in &templates {
        if let Some(str) = path.file_name().and_then(|name| name.to_str()) {
            if str == args.template {
                wanted_template = Some(path);
                println!("Creating project from template \'{str}\'");
            }
        }
    }

    let template = args.template;
    let Some(template_dir) = wanted_template else {
        return Err(anyhow!(
            "The template \'{template}\' does not exist! Found templates:\n{templates:#?}"
        ));
    };
    let tmp_dir = format!("/tmp/{template}");

    copy_dir_all(template_dir, &tmp_dir)?;

    println!("Moving to temporary project");
    std::env::set_current_dir(&tmp_dir).expect("Could not change to temp directory!");

    println!("Opening {editor}...");
    Command::new(editor)
        .arg(&tmp_dir)
        .spawn()?
        .wait()
        .expect("Failed to wait for editor!");

    Ok(())
}

// copied from https://stackoverflow.com/a/65192210/15425442
fn copy_dir_all(from: impl AsRef<Path>, to: impl AsRef<Path>) -> anyhow::Result<()> {
    fs::create_dir(&to)?;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            copy_dir_all(path, to.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(path, to.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

fn get_templates(dirs: &ProjectDirs) -> anyhow::Result<Vec<PathBuf>> {
    let mut templates = Vec::new();
    // should be ~/.local/share/atmpt/*
    for entry in fs::read_dir(dirs.data_dir())? {
        let path = entry?.path();

        if path.is_dir() {
            templates.push(path);
        }
    }

    Ok(templates)
}
