pub mod build;
pub mod list;

use clap::{Parser, Subcommand};
use mx_error::Result;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Root {
    #[command(subcommand)]
    command: RootCommand,
}

impl Root {
    pub fn path(&self) -> &str {
        self.command.path()
    }

    pub fn run(&self) -> Result<()> {
        self.command.run()
    }
}

#[derive(Debug, Subcommand)]
pub enum RootCommand {
    Build(build::Cmd),
    List(list::Cmd),
}

impl RootCommand {
    pub fn path(&self) -> &str {
        match self {
            Self::Build(cmd) => cmd.path(),
            Self::List(cmd) => cmd.path(),
        }
    }

    pub fn run(&self) -> Result<()> {
        match self {
            Self::Build(cmd) => cmd.run(),
            Self::List(cmd) => cmd.run(),
        }
    }
}
