use std::{env, fs, io, path::Path, process::Command};

use anyhow::{bail, Ok};
use chrono::Local;
use clap::Parser;
use directories_next::ProjectDirs;
use templates::Templates;

pub mod templates;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Atmpt {
    #[command(flatten)]
    required: RequiredArgs,

    #[arg(short, long, env = "VISUAL", help = "Use given editor for this run")]
    editor: Option<String>,
}

#[derive(Debug, Parser)]
#[group(required = true)]
pub struct RequiredArgs {
    template: Option<String>,

    #[arg(short = 'd', long = "template-dir", help = "Output template directory")]
    list_template_dir: bool,

    #[arg(short, long, help = "List available templates")]
    list_templates: bool,
}

impl Atmpt {
    pub fn parse_with(dirs: &ProjectDirs) -> anyhow::Result<()> {
        let args = Self::parse();
        let req = args.required;
        let data_dir = dirs.data_dir();

        if let Some(template) = req.template {
            let Some(editor) = args.editor else {
                bail!("No editor to use!"); // really should not happen
            };

            try_template(&template, &editor, data_dir)?;
        } else if req.list_template_dir {
            println!("{}", data_dir.display());
        } else {
            println!("{}", Templates::try_from(data_dir)?);
        }

        Ok(())
    }
}

fn try_template(template: &str, editor: &str, data_dir: &Path) -> anyhow::Result<()> {
    let templates = Templates::try_from(data_dir)?;
    let wanted_dir = templates.find(template)?;

    let time = Local::now().format("%Y_%m_%d-%H_%M_%S");
    let tmp_dir = env::temp_dir()
        .join("atmpt") // store tmp projects in folder
        .join(format!("{template}_{time}"));
    copy_dir_recursively(wanted_dir, &tmp_dir)?;

    std::env::set_current_dir(&tmp_dir).expect("Could not change to temp directory!");
    Command::new(editor)
        .arg(&tmp_dir)
        .spawn()?
        .wait()
        .expect("Could not launch editor!");

    if ask_y_n("Would you like to delete this project?")? {
        fs::remove_dir_all(&tmp_dir)?;
        println!("Deleted.")
    } else {
        println!("Saved as {tmp_dir:?}.");
    }

    Ok(())
}

fn ask_y_n(question: &str) -> anyhow::Result<bool> {
    println!("{question} (Y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.to_lowercase().trim() {
        "" => Ok(true), // default to yes if only enter is pressed
        "y" => Ok(true),
        "n" => Ok(false),
        _ => ask_y_n(question),
    }
}

// modified from https://stackoverflow.com/a/65192210/15425442
fn copy_dir_recursively(from: &Path, to: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(to)?;

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
