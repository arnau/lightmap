use anyhow::Result;
use clap::{AppSettings, Clap};
use lightmap::Package;
use std::path::PathBuf;

/// Builds a DOT representation of the given SQLite database.
#[derive(Debug, Clap)]
#[clap(name = "lightmap", version, global_setting(AppSettings::ColoredHelp))]
struct Cli {
    /// Path to the SQLite database.
    #[clap(value_name = "path")]
    path: PathBuf,
}

impl Cli {
    pub fn run(&self) -> Result<String> {
        let mut package = Package::new(&self.path)?;

        package.to_dot()
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.run() {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }
}
