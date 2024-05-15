use std::{borrow::Cow, fs::File, io::BufReader, path::PathBuf};

use anyhow::{bail, Context};
use atmpt::{get_session_path, session::Session, templates::Templates, Atmpt};
use clap::Parser;
use directories::ProjectDirs;

fn main() -> anyhow::Result<()> {
    let Some(dirs) = ProjectDirs::from("me", "marcelohdez", "Atmpt") else {
        bail!("Could not generate any directories for this OS!");
    };

    let args = Atmpt::parse();
    let action = args.after_action();

    let mut data_dir = Cow::Borrowed(dirs.data_dir());
    if let Some(new_dir) = &args.data_dir {
        data_dir = Cow::Owned(PathBuf::from(new_dir));
    };

    if let Some(template) = args.required.template {
        let Some(editor) = args.editor else {
            bail!("No editor to use! Set your $VISUAL variable or pass a command to --editor");
        };

        atmpt::try_template(&template, &editor, &data_dir, action)?;
    } else if args.required.retry {
        let file = File::open(get_session_path())
            .context("Could not open session file, have you run atmpt recently?")?;
        let session: Session =
            serde_json::from_reader(BufReader::new(file)).context("Failed to read session file")?;
        let Some(editor) = args.editor else {
            bail!("No editor to use! Set your $VISUAL variable or pass a command to --editor");
        };

        atmpt::try_template(&session.last_template, &editor, &data_dir, action)?;
    } else if args.required.list_template_dir {
        println!("{}", data_dir.display());
    } else {
        println!("{}", Templates::try_from(data_dir.as_ref())?);
    }

    Ok(())
}
