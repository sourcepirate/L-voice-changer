
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;

use std::path::PathBuf;
use std::fs::canonicalize;
use clap::{App, Arg};
use std::process::exit;

mod temp;
mod command;

static APP_NAME: &str = "Lvoice";
static VERSION: &str = "1.0.0";


fn main() {
    env_logger::init();
    let app = App::new(APP_NAME)
        .version(VERSION)
        .arg(Arg::with_name("in").required(true).takes_value(true))
        .arg(Arg::with_name("out").required(true).takes_value(true))
        .get_matches();
    let input_file = app.value_of("in").unwrap();
    let output_file = app.value_of("out").unwrap();
    let mut input_path = PathBuf::from(input_file);
    let mut output_path = PathBuf::from(output_file);
    if input_path.is_relative() {
        input_path = canonicalize(input_path).unwrap();
    }
    if output_path.is_relative() {
        output_path = canonicalize(output_path).unwrap();
    }
    if !output_path.is_dir() {
        println!("Output path must be a directory not file");
        exit(1);
    }
    if input_path.is_dir() {
        println!("Input path must be a file");
        exit(1);
    }
    let lowpass_file = temp::TempAudio::from_source(input_path.to_str().unwrap(),
                                                    command::AudioType::LOWPASS);
    let highpass_file = temp::TempAudio::from_source(input_path.to_str().unwrap(),
                                                     command::AudioType::HIGHPASS);
    lowpass_file.create(input_path.to_str().unwrap());
    highpass_file.create(input_path.to_str().unwrap());
    output_path.push("out.mp3");
    debug!("{:?}", input_path);
    debug!("{:?}", output_path);
    match command::merge_audio(lowpass_file, highpass_file, output_path.to_str().unwrap()) {
        Ok(()) => {println!("Done!")}
        Err(_) => println!("Error"),
    }

}
