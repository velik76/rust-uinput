use std::env;
use std::process::Command;

fn main() {
    let mut parser = env::current_dir().unwrap().into_os_string();
    parser.push("/Python/events_parser.py");

    let output = Command::new("python3").arg(parser).output().expect("failed to execute process");

    let hello = String::from_utf8(output.stderr);
    let result_str;
    match hello {
        Ok(val) => result_str = val,
        Err(_) => panic!("got non UTF-8 data"),
    }
    println!("Result of python call: {}", result_str);
}
