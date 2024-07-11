#![allow(unused)]
#[macro_use]
extern crate nix;
extern crate libc;

use serde_json::Result;
use std::env;
use std::fs;

pub mod test_scenario;
pub mod uinput;

use test_scenario::TestScenario;

fn usage() {
    println!("usage: run it with json file wih scenario settins");
}

fn get_test_scenario(file_path: &str) -> TestScenario {
    // Read JSON file
    let json_contents = fs::read_to_string(file_path).expect("Couldn't find or load config file");

    // Parse the JSON and check if all Ok
    match serde_json::from_str(&json_contents) {
        Ok(x) => {
            println!("JSON file: {} read Ok", file_path);
            x
        }
        Err(err) => {
            println!("Error parsing of JSON file: {}", err);
            std::process::exit(1);
        }
    }
}

fn play_test_scenario(scenario: &TestScenario) {
    println!("Repeats: {} ", scenario.repeats);
    for prog in &*scenario.program {
        print!("type: {}. ", prog.get("type").unwrap());
        print!("param1: {}. ", prog.get("param1").unwrap());
        if prog.contains_key("param2") {
            print!("param2: {}", prog.get("param2").unwrap());
        }
        println!("");
    }
}

fn main() {
    // TODO: Just test code
    uinput::ioctl_test();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        std::process::exit(1);
    }

    let scenario = get_test_scenario(&args[1]);

    // uinput::init(&scenario);
    // play_test_scenario(&scenario);
    // std::process::exit(0);
}
