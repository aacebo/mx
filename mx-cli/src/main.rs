use clap::Parser;
use mx_core::config;

mod cmds;
mod context;

fn main() {
    let cmd = cmds::Root::parse();
    let path = std::path::Path::new(cmd.path());
    let config = match config::ProjectConfig::from_file(path) {
        Err(err) => panic!("{}", err),
        Ok(v) => v,
    };

    println!("{:#?}", config);

    if let Err(err) = cmd.run() {
        panic!("{}", err);
    }
}
