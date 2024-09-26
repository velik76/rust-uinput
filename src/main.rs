#![allow(unused)]
#[macro_use]
extern crate nix;

use core::str;
use input_linux_sys::*;
use serde_yaml::*;
use std::{env, fs, io, thread, time};
pub mod keys_enum;
pub mod test_scenario;
pub mod uinput;

use test_scenario::TestScenario;

fn usage() {
    println!("usage: run it with yaml file wih scenario settings");
}

/// Reads out and parses the test scenario file. Now uses test scenario as yaml file
///
/// # Arguments
///
/// * `file_path` - Path to the test scenario file
///
fn get_test_scenario(file_path: &str) -> TestScenario {
    // Read file
    let file_contents = fs::read_to_string(file_path).expect("Couldn't find or load config file");

    // Parse the scenario and check if all Ok
    let result = serde_yaml::from_str(&file_contents);
    match result {
        Ok(x) => {
            println!("Test scenario file: {} read Ok", file_path);
            x
        }
        Err(err) => {
            println!("Error parsing of config file: {}", err);
            std::process::exit(1);
        }
    }
}

/// Brief.
///
/// Description.
///
/// * `foo` - Text about foo.
/// * `bar` - Text about bar.
fn play_test_scenario(scenario: &TestScenario, file: &std::fs::File) {
    for i in 0..scenario.repeats {
        for prog in &*scenario.program {
            match prog.get("type").unwrap().as_str() {
                "key_event" => {
                    let press_s = prog.get("param1").unwrap();
                    let code_s = prog.get("param2").unwrap();
                    let press_b = if press_s.as_str() == "press" { true } else { false };
                    let key_code = code_s.parse::<i32>().unwrap();
                    uinput::press_key(file, key_code, press_b);
                }
                "delay" => {
                    let delay_s = prog.get("param1").unwrap();
                    let delay_u64 = delay_s.parse::<u64>().unwrap();
                    thread::sleep(time::Duration::from_millis(delay_u64));
                }
                _ => println!("something else!"),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        std::process::exit(1);
    }

    // Read scenario from yaml file
    let scenario = get_test_scenario(&args[1]);

    // Read scenario from yaml file
    let file = uinput::setup();
    play_test_scenario(&scenario, &file);
    uinput::teardown(&file);
    std::process::exit(0);
}
