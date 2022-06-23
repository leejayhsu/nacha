#![allow(unused)]

mod lib;
use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Env;

/// Parse a NACHA file into a rust object
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// path to your NACHA file
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "warning")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    let nacha_file = lib::NachaFile::new(content);
    let serialized = serde_json::to_string_pretty(&nacha_file).unwrap();
    println!("{}", serialized);
    Ok(())
}
