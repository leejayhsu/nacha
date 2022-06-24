#![allow(unused)]

mod lib;
use std::{ffi::OsStr, fs::File, io::Write, path::PathBuf};

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
    /// name of output file, leave unset for stdout. json and yaml are supported if extension is provided, If no extension is detected, defaults to json.
    #[clap(parse(from_os_str))]
    output: Option<std::path::PathBuf>,
}

impl Cli {
    fn output(self, data: &lib::NachaFile) {
        let as_json = serde_json::to_string_pretty(data).unwrap();
        let as_yaml = serde_yaml::to_string(data).unwrap();

        if let Some(output_path) = self.output {
            let ext = output_path.extension().and_then(OsStr::to_str);
            match ext {
                Some("json") => {
                    let mut output_file = File::create(output_path).unwrap();
                    write!(output_file, "{}", as_json);
                }
                Some("yaml") => {
                    let mut output_file = File::create(output_path).unwrap();
                    write!(output_file, "{}", as_yaml);
                }
                Some("yml") => {
                    let mut output_file = File::create(output_path).unwrap();
                    write!(output_file, "{}", as_yaml);
                }
                None => {}
                _ => {}
            }
        } else {
            println!("{}", as_json);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "warning")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let cli = Cli::parse();

    let content = std::fs::read_to_string(&cli.path)
        .with_context(|| format!("could not read file `{}`", &cli.path.display()))?;

    let nacha_file = lib::NachaFile::new(content);

    cli.output(&nacha_file);

    Ok(())
}
