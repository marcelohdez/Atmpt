use clap::Parser;

pub mod templates;

pub const EDITOR_KEY: &str = "VISUAL";
pub const ALWAYS_DELETE_KEY: &str = "ATMPT_ALWAYS_DELETE";
pub const TEMPLATE_DIR_KEY: &str = "ATMPT_DATA_DIR";

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Atmpt {
    #[command(flatten)]
    pub required: RequiredArgs,

    #[arg(short, long, env = EDITOR_KEY, help = "Use given editor for this run")]
    pub editor: Option<String>,

    #[arg(short = 'y', long, env = ALWAYS_DELETE_KEY, help = "Autodelete project on exit")]
    pub delete: bool,

    #[arg(long, hide = true, env = TEMPLATE_DIR_KEY)] // override template dir
    pub template_dir: Option<String>,
}

#[derive(Debug, Parser)]
#[group(required = true)]
pub struct RequiredArgs {
    #[arg(group = "main")]
    pub template: Option<String>,

    #[arg(group = "main", short = 'd', long, help = "Output template directory")]
    pub list_template_dir: bool,

    #[arg(group = "main", short, long, help = "List available templates")]
    pub list_templates: bool,
}
