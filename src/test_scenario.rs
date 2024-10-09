use crate::keys_enum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum KeyAction {
    Press,
    Release,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FullEvent {
    KeyEvent { key: keys_enum::Keys, action: KeyAction },
    Delay { duration: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestScenario {
    pub repeats: u32,
    pub program: Vec<FullEvent>,
}
