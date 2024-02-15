pub mod cli;
pub mod templates;

pub use cli::*;
pub use templates::*;

use std::{env, fs, io, path::Path, process::Command};

use anyhow::Context;
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

    Command::new(editor)
        .current_dir(&tmp_dir)
        .arg(".")
        .spawn()
        .context("Failed to launch editor!")?
        .wait()
        .context("Failed waiting for editor!")?;

    if delete || ask_y_n("Would you like to keep this project?")? {
        println!("Saved as {tmp_dir:?}.");
    } else {
        fs::remove_dir_all(&tmp_dir)
            .with_context(|| format!("Failed to remove directory {tmp_dir:?}"))?;
    }

    Ok(())
}

/// Will print out a question and wait for user input in the form of `y` or `n`
/// (returning true if `y`). Any capitalization works. Defaults to `n` on a
/// blank character (e.g. just pressing Enter).
///
/// Will return Err(_) if stdio fails to open.
fn ask_y_n(question: &str) -> anyhow::Result<bool> {
    println!("{question} (y/N)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to open stdio!")?;

    match input.to_lowercase().trim() {
        "y" => Ok(true),
        "n" | "" => Ok(false),  // default to no if no input
        _ => ask_y_n(question), // redo if wrong input
    }
}

// modified from https://stackoverflow.com/a/65192210/15425442
fn copy_dir_recursively(from: &Path, to: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(to).with_context(|| format!("Failed to create directory {to:?}!"))?;

    let entries = fs::read_dir(from).with_context(|| format!("Failed to copy {from:?}!"))?;

    for entry in entries {
        let entry = entry.context("Failed to unwrap entry to copy!")?;
        let path = entry.path();

        if path.is_dir() {
            copy_dir_recursively(&path, &to.join(entry.file_name()))?;
        } else {
            fs::copy(path, to.join(entry.file_name()))
                .with_context(|| format!("Failed to copy {entry:?}!"))?;
        }
    }

    Ok(())
}
