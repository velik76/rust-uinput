#![allow(unused)]
#[macro_use]
extern crate nix;

use nix::libc::exit;

use std::{env, fs, io, result, thread, time};
use test_scenario::{FullEvent, KeyAction, MouseAction, TestScenario};

pub mod keys_enum;
pub mod test_scenario;
pub mod uinput;

fn usage() {
    println!("usage: run it with yaml file wih scenario settings");
}

fn get_test_scenario(file_path: &str) -> result::Result<TestScenario, String> {
    // Read file
    let file_contents = fs::read_to_string(file_path).expect("Couldn't find or load config file");

    // Parse the scenario and check if all Ok
    match serde_yaml::from_str(&file_contents) {
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
                    let press_b = match action {
                        KeyAction::Press => true,
                        KeyAction::Release => false,
                    };
                    uinput::press_key(file, *key as i32, press_b);
                }
                FullEvent::KeyPressReleaseEvent { key } => {
                    let key_i32 = *key as i32;
                    uinput::press_key(file, *key as i32, true);
                    uinput::press_key(file, *key as i32, false);
                }
                FullEvent::MouseEvent { action, x, y } => match action {
                    MouseAction::Set => {
                        uinput::mouse_set(file, *x, *y);
                    }
                    MouseAction::Move => {
                        println!("MouseAction::Move not implemented yet");
                    }
                },
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

    // Read scenario from yaml file
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
    match (uinput::setup(1920, 1200)) {
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
