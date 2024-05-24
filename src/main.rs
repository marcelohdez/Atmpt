use std::{borrow::Cow, fs::File, io::BufReader, path::PathBuf, str::FromStr};

use anyhow::{bail, Context};
use atmpt::{get_session_path, session::Session, templates::Templates, Atmpt};
use clap::Parser;
use directories::ProjectDirs;

fn main() -> anyhow::Result<()> {
    let Some(dirs) = ProjectDirs::from("me", "marcelohdez", atmpt::PROGRAM_NAME) else {
        bail!("Could not generate any directories for this OS!");
    };

    let args = Atmpt::parse();
    let action = args.after_action();

    let tmp_dir = args.tmp_dir.map(|s| PathBuf::from_str(&s)).transpose()?;
    let data_dir = match &args.data_dir {
        Some(new_dir) => Cow::Owned(PathBuf::from_str(new_dir)?),
        None => Cow::Borrowed(dirs.data_dir()),
    };

    if args.required.list_template_dir {
        println!("{}", data_dir.display());
    } else if args.required.list_templates {
        println!("{}", Templates::try_from(data_dir.as_ref())?);
    } else if args.required.previous {
        let session = Session::from_file(&get_session_path(&tmp_dir))?;
        if !session.previous_attempt.exists() {
            bail!(
                "Last modified attempt, {:?} does not exist! Did it move?\nMake a new attempt!",
                session.previous_attempt
            );
        }

        atmpt::summon_and_wait(&args.editor, &session.previous_attempt)?;
    } else {
        let template = match args.required.template {
            Some(t) => t,
            // assume retry option
            None => {
                let file = File::open(get_session_path(&tmp_dir))
                    .context("Could not open session file, have you run atmpt recently?")?;
                let session: Session = serde_json::from_reader(BufReader::new(file))
                    .context("Failed to read session file!")?;

                session.last_template
            }
        };

        atmpt::try_template(&template, &args.editor, &data_dir, &tmp_dir, action)?;
    }

    Ok(())
}
