use crate::term::run;
use anyhow::{Context, Result};
use clap::Parser;
use env_logger::Env;
use std::time::Duration;
use std::{ffi::OsStr, fs::File, io::Write};
mod app;
mod lib;
mod term;
mod ui;

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
    fn output(&self, data: &lib::NachaFile) {
        if let Some(output_path) = &self.output {
            let ext = output_path.extension().and_then(OsStr::to_str);
            match ext {
                Some("json") => {
                    let mut output_file = File::create(output_path).unwrap();
                    write!(output_file, "{}", data.as_json()).expect("could not write json file");
                }
                Some("yaml") => {
                    let mut output_file = File::create(output_path).unwrap();
                    write!(output_file, "{}", data.as_yaml()).expect("could not write yaml file");
                }
                Some("yml") => {
                    let mut output_file = File::create(output_path).unwrap();
                    write!(output_file, "{}", data.as_yaml()).expect("could not write yaml file");
                }
                None => {}
                _ => {}
            }
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

    let mut nacha_file = lib::NachaFile::new(content);

    cli.output(&nacha_file);

    if let None = cli.output {
        run(Duration::from_millis(1000), &mut nacha_file)?;
    }

    Ok(())
}
