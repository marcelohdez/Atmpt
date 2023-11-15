use std::{fs, io::Result, path::PathBuf};

use clap::CommandFactory;
use clap_complete::{generate_to, Shell};

include!("src/cli.rs");

fn main() -> Result<()> {
    let mut cli = Atmpt::command();

    let dir = PathBuf::from("completions");
    if !dir.is_dir() {
        fs::create_dir(&dir)?;
    }

    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {
        let comp_file = generate_to(shell, &mut cli, "atmpt", &dir)?;
        println!("cargo:warning=generated completion for {shell}: {comp_file:?}");
    }

    Ok(())
}
