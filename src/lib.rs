pub mod cli;
pub mod templates;

pub use cli::*;
pub use templates::*;

use std::{env, fs, io, path::Path, process::Command};

use anyhow::bail;
use chrono::Local;

pub fn try_template(
    template: &str,
    editor: &str,
    delete: bool,
    data_dir: &Path,
) -> anyhow::Result<()> {
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

    if delete || ask_y_n("Would you like to keep this project?")? {
        println!("Saved as {tmp_dir:?}.");
    } else {
        fs::remove_dir_all(&tmp_dir)?;
    }

    Ok(())
}

fn ask_y_n(question: &str) -> anyhow::Result<bool> {
    println!("{question} (y/N)");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.to_lowercase().trim() {
        "y" => Ok(true),
        "n" | "" => Ok(false),  // default to no if no input
        _ => ask_y_n(question), // redo if wrong input
    }
}

// modified from https://stackoverflow.com/a/65192210/15425442
fn copy_dir_recursively(from: &Path, to: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(to)?;

    let Ok(entries) = fs::read_dir(from) else {
        bail!("Could not read dir to copy from:\n{from:?}\nDoes it exist?")
    };

    for entry in entries {
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
