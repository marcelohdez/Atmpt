use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Atmpt {
    pub template: String,
}
