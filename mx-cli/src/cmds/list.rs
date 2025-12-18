use clap::Parser;
use mx_error::Result;

#[derive(Debug, Parser)]
#[command(author, version, about = "list projects")]
pub struct Cmd {
    path: Option<String>,
}

impl Cmd {
    pub fn path(&self) -> &str {
        match &self.path {
            None => ".",
            Some(v) => v,
        }
    }

    pub fn run(&self) -> Result<()> {
        println!("hello, world! {:#?}", self);
        Ok(())
    }
}
