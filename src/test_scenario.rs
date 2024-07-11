use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestScenario {
    pub repeats: i32,
    pub program: Vec<HashMap<String, String>>,
}
