use std::env;
use std::process;

struct Config {
    input_video: String,
    input_music: String,
    output: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let input_video = args[1].clone();
        let input_music = args[2].clone();
        let output = args[3].clone();

        Ok(Config {
            input_video,
            input_music,
            output,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
