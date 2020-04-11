use std::env;
use std::process;
use std::process::{Command, Stdio};

struct Flag {
    input_video: String,
    input_audio: String,
    input_time: String,
    output: String,
}

impl Flag {
    pub fn new(args: &[String]) -> Result<Flag, &'static str> {
        if args.len() < 5 {
            return Err("not enough arguments");
        }

        let input_video = args[1].clone();
        let input_audio = args[2].clone();
        let input_time = args[3].clone();
        let output = args[4].clone();

        Ok(Flag {
            input_video,
            input_audio,
            input_time,
            output,
        })
    }

    pub fn format_command(&self) -> String {
        format!("ffmpeg -i {input_v} -itsoffset {input_t} -i {input_a} -c:v copy -map 0:v:0 -map 1:a:0 {output}.mp4",
                input_v = self.input_video,
                input_t = format_hhmmss(self.input_time.parse::<usize>().unwrap()),
                input_a = self.input_audio,
                output = self.output
                )
    }
}

fn format_hhmmss(target_seconds: usize) -> String {
    let target_minutes = target_seconds / 60;
    let seconds = target_seconds % 60;
    let hour = target_minutes / 60;
    let minutes = target_minutes % 60;

    return format!("{:02}:{:02}:{:02}", hour, minutes, seconds);
}

pub fn exec_cmd(command: String) {
    let mut combine = Command::new("/bin/sh")
        .args(&["-c", &command])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to execute combine");
    {
        let _stdin = combine.stdin.as_mut().expect("filed to get stdin");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Flag::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let command = config.format_command();
    println!("{}", command);

    exec_cmd(command)
}
