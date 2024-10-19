use regex::Regex;
use std::io::prelude::*;
use std::path::Path;
use std::{env, fs};

fn main() {
    let input = Path::new(&env::current_dir().unwrap()).join("src/resource.rs");
    let output = Path::new(&env::current_dir().unwrap()).join("src/keys_enum.rs");
    let mut file = fs::File::create(output).unwrap();

    // Write header in file
    file.write_all(
        b"/// AUTOMATICALLY GENERATED. DO NOT EDIT!\n
use serde::{Deserialize, Serialize};\n
#[allow(non_camel_case_types)]\n
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Keys {
\tBTN_LEFT = 0x110,
\tBTN_RIGHT = 0x111,\n",
    )
    .unwrap();

    for line in fs::read_to_string(input).unwrap().lines() {
        let re = Regex::new(r"([ ]+)").unwrap();
        let parts: Vec<&str> = re.split(line).collect();
        if parts.len() < 6 {
            continue;
        }
        if parts[0] == "pub" && parts[2].starts_with("KEY_") {
            let mut key = parts[2].to_string();
            // Remove ':' at the end
            if key.ends_with(":") {
                key.pop();
            }
            let mut value = parts[5].to_string();
            // Remove ';' at the end
            if value.ends_with(";") {
                value.pop();
            }
            // Some values has string. Try to convert into int, in error case ignore these values via continue
            if !value.starts_with("0x") {
                match value.parse::<i32>() {
                    Err(_) => continue,
                    _ => {}
                }
            }

            let s_to_file = "\t".to_string() + &key + " = " + &value + ",\n";
            file.write_all(s_to_file.as_bytes()).unwrap();
        }
    }
    file.write_all(b"}\n").unwrap();
}
