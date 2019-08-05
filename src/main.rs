use std::env;
use std::process;

struct Config {
    input_video: String,
    input_music: String,
    input_time: String,
    output: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 {
            return Err("not enough arguments");
        }

        let input_video = args[1].clone();
        let input_music = args[2].clone();
        let input_time = args[3].clone();
        let output = args[4].clone();

        Ok(Config {
            input_video,
            input_music,
            input_time,
            output,
        })
    }

    pub fn format_command(&self) -> String {
        format!("ffmpeg -i {input_v} -itsoffset {input_t} -i {input_m} -c copy -map 0:v:0 -map 1:a:0 {output}",
                input_v = self.input_video,
                input_t = self.input_time,
                input_m = self.input_music,
                output = self.output
                )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let command = config.format_command();
    println!("{}", command)
}
