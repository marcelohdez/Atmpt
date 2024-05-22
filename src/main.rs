use std::{
    borrow::Cow,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
    str::FromStr,
};

use anyhow::{bail, Context};
use atmpt::{get_atmpt_dir, get_session_path, session::Session, templates::Templates, Atmpt};
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
        atmpt::summon_and_wait(&args.editor, &last_modified_attempt(&tmp_dir)?)?;
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

fn last_modified_attempt(tmp_dir: &Option<PathBuf>) -> anyhow::Result<PathBuf> {
    let atmpt_dir = get_atmpt_dir(tmp_dir);
    if !atmpt_dir.exists() {
        bail!("Could not find atmpt folder, have you run atmpt recently?");
    }

    let entries = fs::read_dir(atmpt_dir.as_ref())?;
    let mut last = None;
    for entry in entries {
        let entry = entry?;

        let metadata = entry.metadata()?;
        if !metadata.is_dir() {
            continue;
        }

        let new_time = metadata.modified()?;
        if let Some((_, time)) = last {
            if new_time <= time {
                continue;
            }
        }

        last = Some((entry.path(), new_time));
    }

    if let Some((path, _)) = last {
        Ok(path)
    } else {
        bail!("Could not find last modified attempt, have you saved any?")
    }
}
