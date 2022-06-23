mod lib;
use env_logger::Env;

fn main() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let file = std::fs::File::open("example.ach").unwrap();
    let reader = std::io::BufReader::new(file);
    let mut nacha_file: lib::NachaFile = Default::default();
    nacha_file.parse(reader)
}
