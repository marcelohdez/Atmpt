use clap::Parser;

pub const EDITOR_KEY: &str = "VISUAL";
pub const ALWAYS_DELETE_KEY: &str = "ATMPT_ALWAYS_DELETE";
pub const TEMPLATE_DIR_KEY: &str = "ATMPT_DATA_DIR";

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Atmpt {
    #[command(flatten)]
    pub required: RequiredArgs,

    #[arg(short, long, env = EDITOR_KEY, help = "Editor to use")]
    pub editor: Option<String>,

    #[arg(short = 'n', long, hide_env = true, env = ALWAYS_DELETE_KEY, help = "Delete project on exit")]
    pub delete: bool,

    #[arg(long, hide_env = true, env = TEMPLATE_DIR_KEY, help = "Override template dir")]
    pub new_template_dir: Option<String>,
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
