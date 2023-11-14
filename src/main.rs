use anyhow::bail;
use atmpt::Atmpt;
use directories_next::ProjectDirs;

fn main() -> anyhow::Result<()> {
    let Some(dirs) = ProjectDirs::from("me", "marcelohdez", "Atmpt") else {
        bail!("Could not generate any directories for this OS!");
    };

    Atmpt::parse_with(&dirs)
}
