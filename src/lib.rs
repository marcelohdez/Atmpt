use std::{env, ffi::OsStr, fs, io, path::Path, process::Command};

use anyhow::Ok;
use clap::Parser;
use directories_next::ProjectDirs;
use templates::Templates;

pub mod templates;

#[derive(Debug, Parser)]
#[command(author, version, about)]
#[group(required = true)]
pub struct Atmpt {
    #[arg(group = "main")]
    template: Option<String>,

    #[arg(
        group = "main",
        short = 'd',
        long = "template-dir",
        conflicts_with = "template",
        help = "Output template directory"
    )]
    list_template_dir: bool,
}

impl Atmpt {
    pub fn parse_with(editor: &OsStr, dirs: &ProjectDirs) -> anyhow::Result<()> {
        let args = Self::parse();
        let data_dir = dirs.data_dir();

        match args.template {
            Some(template) => try_template(&template, editor, data_dir)?,
            None => output_dir(data_dir),
        }

        Ok(())
    }
}

fn output_dir(path: &Path) {
    if let Some(str) = path.to_str() {
        println!("{str}");
    }
}

fn try_template(template: &str, editor: &OsStr, data_dir: &Path) -> anyhow::Result<()> {
    let templates = Templates::try_from(data_dir)?;
    let wanted_dir = templates.find(template)?;

    let tmp_dir = env::temp_dir().join(template);
    copy_dir_recursively(wanted_dir, &tmp_dir)?;

    std::env::set_current_dir(&tmp_dir).expect("Could not change to temp directory!");
    Command::new(editor)
        .arg(&tmp_dir)
        .spawn()?
        .wait()
        .expect("Could not launch editor!");

    if ask_y_n("Would you like to delete this project?")? {
        fs::remove_dir_all(&tmp_dir)?;
    }

    Ok(())
}

fn ask_y_n(question: &str) -> anyhow::Result<bool> {
    println!("{question} (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim() {
        "y" => Ok(true),
        "n" => Ok(false),
        _ => ask_y_n(question),
    }
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
