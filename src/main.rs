use std::ffi::{OsStr, OsString};

use anyhow::{bail, Ok};
use atmpt::Atmpt;
use directories_next::ProjectDirs;

fn main() -> anyhow::Result<()> {
    let editor = get_editor()?;
    let Some(dirs) = ProjectDirs::from("me", "marcelohdez", "Atmpt") else {
        bail!("Could not generate any directories for this OS!");
    };

    Atmpt::parse_with(&editor, &dirs)
}

fn get_editor() -> anyhow::Result<OsString> {
    let editor_key = OsStr::new("EDITOR");

    for (k, v) in std::env::vars_os() {
        if k == editor_key {
            return Ok(v);
        }
    }

    bail!("No editor could be found! Please set your $EDITOR environment variable")
}
