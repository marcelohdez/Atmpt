pub mod cli;
pub mod session;
pub mod templates;

pub use cli::*;
use session::Session;
pub use templates::*;

use std::{
    borrow::Cow,
    env,
    fs::{self, File},
    io::{self, BufWriter},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context, Ok};
use chrono::Local;

pub fn get_atmpt_dir(tmp_dir: &Option<PathBuf>) -> Cow<PathBuf> {
    match tmp_dir {
        Some(d) => Cow::Borrowed(d),
        None => Cow::Owned(env::temp_dir().join("atmpt")),
    }
}

pub fn get_session_path(tmp_dir: &Option<PathBuf>) -> PathBuf {
    get_atmpt_dir(tmp_dir).join("session.json")
}

pub fn try_template(
    template: &str,
    editor: &str,
    data_dir: &Path,
    tmp_dir: &Option<PathBuf>,
    action: Option<AfterAction>,
) -> anyhow::Result<()> {
    let templates = Templates::try_from(data_dir)?;
    let wanted_dir = templates.find(template)?;
    let projects_dir = get_atmpt_dir(tmp_dir);

    let time = Local::now().format("%Y_%m_%d-%H_%M_%S");
    let project_dir = projects_dir.join(format!("{template}_{time}"));

    copy_dir_recursively(wanted_dir, &project_dir)?;
    if let Err(e) = summon_and_wait(editor, &project_dir) {
        remove_attempt(&project_dir)?;
        bail!(e);
    }

    // save session data to file
    let file = File::create(get_session_path(tmp_dir))?;
    let session = Session {
        last_template: template.to_owned(),
    };
    serde_json::to_writer(BufWriter::new(file), &session)?;

    if should_keep(action)? {
        println!("Saved as {project_dir:?}.");
    } else {
        remove_attempt(&project_dir)?;
    }

    Ok(())
}

pub fn summon_and_wait(editor: &str, cwd: &Path) -> anyhow::Result<()> {
    Command::new(editor)
        .current_dir(cwd)
        .arg(".")
        .spawn()
        .context("Failed to launch editor!")?
        .wait()
        .context("Failed waiting for editor!")?;

    Ok(())
}

fn should_keep(action: Option<AfterAction>) -> anyhow::Result<bool> {
    match action {
        Some(action) => Ok(action == AfterAction::Keep),
        None => ask_y_n("Would you like to keep this code?"),
    }
}

fn remove_attempt(tmp_dir: &Path) -> anyhow::Result<()> {
    fs::remove_dir_all(tmp_dir)
        .with_context(|| format!("Failed to remove directory {tmp_dir:?}"))?;

    println!("Deleted.");
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
