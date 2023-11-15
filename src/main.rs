use std::{
    borrow::Cow,
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::bail;
use atmpt::{templates::Templates, Atmpt};
use chrono::Local;
use clap::Parser;
use directories_next::ProjectDirs;

fn main() -> anyhow::Result<()> {
    let Some(dirs) = ProjectDirs::from("me", "marcelohdez", "Atmpt") else {
        bail!("Could not generate any directories for this OS!");
    };

    let args = Atmpt::parse();
    let req = args.required;

    let mut data_dir = Cow::Borrowed(dirs.data_dir());
    if let Some(new_dir) = args.template_dir {
        *data_dir.to_mut() = PathBuf::from(new_dir);
    };

    if let Some(template) = req.template {
        let Some(editor) = args.editor else {
            bail!("No editor to use! Set your $VISUAL variable or pass a command to --editor");
        };

        try_template(&template, &editor, args.delete, &data_dir)?;
    } else if req.list_template_dir {
        println!("{}", data_dir.display());
    } else {
        println!("{}", Templates::try_from(data_dir.as_ref())?);
    }

    Ok(())
}

fn try_template(template: &str, editor: &str, delete: bool, data_dir: &Path) -> anyhow::Result<()> {
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

    if delete || ask_y_n("Would you like to delete this project?")? {
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
