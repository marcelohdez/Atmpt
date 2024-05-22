use clap::Parser;

pub const EDITOR_KEY: &str = "VISUAL";
pub const ALWAYS_DELETE_KEY: &str = "ATMPT_ALWAYS_DELETE";
pub const ALWAYS_KEEP_KEY: &str = "ATMPT_ALWAYS_KEEP";
pub const DATA_DIR_KEY: &str = "ATMPT_DATA_DIR";
pub const TMP_DIR_KEY: &str = "ATMPT_TMP_DIR";

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Atmpt {
    #[command(flatten)]
    pub required: RequiredArgs,

    #[arg(short, long, env = EDITOR_KEY, help = "Editor to use")]
    pub editor: String,

    #[arg(
        short = 'n',
        long,
        hide_env = true,
        conflicts_with = "keep",
        env = ALWAYS_DELETE_KEY,
        help = "Delete project on exit"
    )]
    pub delete: bool,

    #[arg(
        short = 'y',
        long,
        hide_env = true,
        env = ALWAYS_KEEP_KEY,
        help = "Keep project on exit"
    )]
    pub keep: bool,

    #[arg(long, hide_env = true, env = DATA_DIR_KEY, help = "Override templates directory")]
    pub data_dir: Option<String>,

    #[arg(long, short, env = DATA_DIR_KEY, help = "Override attempts directory")]
    pub tmp_dir: Option<String>,
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

    #[arg(group = "main", short, long, help = "Retry last template")]
    pub retry: bool,

    #[arg(group = "main", short, long, help = "Open last modified attempt")]
    pub previous: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AfterAction {
    Keep,
    Delete,
}

impl Atmpt {
    pub fn after_action(&self) -> Option<AfterAction> {
        if self.keep {
            return Some(AfterAction::Keep);
        }

        if self.delete {
            return Some(AfterAction::Delete);
        }

        None
    }
}
