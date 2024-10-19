use crate::keys_enum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum KeyAction {
    Press,
    Release,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum MouseAction {
    Set,
    Move,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum FullEvent {
    KeyEvent { key: keys_enum::Keys, action: KeyAction },
    KeyPressReleaseEvent { key: keys_enum::Keys },
    MouseEvent { action: MouseAction, x: i32, y: i32 },
    Delay { duration: u32 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestScenario {
    pub display_width: u32,
    pub display_height: u32,
    pub repeats: u32,
    pub program: Vec<FullEvent>,
}
