#![allow(unused)]
#[macro_use]
extern crate nix;

use core::str;
use input_linux_sys::*;
use nix::libc::exit;
use serde_yaml::*;
use std::{env, fs, io, result, thread, time};
use test_scenario::KeyAction;
pub mod keys_enum;
pub mod test_scenario;
pub mod uinput;

use test_scenario::FullEvent;
use test_scenario::TestScenario;

fn usage() {
    println!("usage: run it with yaml file wih scenario settings");
}

fn get_test_scenario(file_path: &str) -> result::Result<TestScenario, String> {
    // Read file
    let file_contents = fs::read_to_string(file_path).expect("Couldn't find or load config file");

    // Parse the scenario and check if all Ok
    let result = serde_yaml::from_str(&file_contents);
    match result {
        Ok(x) => {
            println!("Test scenario file: {} read Ok", file_path);
            result::Result::Ok(x)
        }
        Err(err) => result::Result::Err(err.to_string()),
    }
}

fn play_test_scenario(scenario: &TestScenario, file: &std::fs::File) {
    for i in 0..scenario.repeats {
        for prog in &*scenario.program {
            match prog {
                FullEvent::KeyEvent { key, action } => {
                    let key_i32 = *key as i32;
                    let mut press_b = false;
                    match action {
                        KeyAction::Press => press_b = true,
                        KeyAction::Release => {}
                    }
                    uinput::press_key(file, *key as i32, press_b);
                }
                FullEvent::Delay { duration } => {
                    let delay_u64 = *duration as u64;
                    thread::sleep(time::Duration::from_millis(delay_u64));
                }
            }
        }
    }
}

fn main() {
    // Get and check arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        std::process::exit(1);
    }

    // Read scenario from given yaml file
    let mut scenario: TestScenario;
    match (get_test_scenario(&args[1])) {
        Ok(x) => scenario = x,
        Err(text) => {
            println!("Error parsing scenario file: {}", text);
            std::process::exit(2)
        }
    }

    // Setup uinput
    let mut file: std::fs::File;
    match (uinput::setup()) {
        Ok(x) => file = x,
        Err(text) => {
            println!("Error setup uinput: {}", text);
            std::process::exit(3)
        }
    }

    // Play scenario
    play_test_scenario(&scenario, &file);

    // Finish
    uinput::teardown(&file);
    std::process::exit(0);
}
