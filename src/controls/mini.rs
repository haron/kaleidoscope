use macroquad::prelude::*;

use crate::controls::models::Command;

use super::Keyboard;

pub struct MiniKeyboard {}

impl Keyboard for MiniKeyboard {
    fn process_input(&self) -> Option<Command> {
        match get_last_key_pressed() {
            Some(KeyCode::Escape) => Some(Command::Quit),
            Some(KeyCode::Key1) => Some(Command::IncreaseCycle),
            Some(KeyCode::Key2) => Some(Command::ResetCycle),
            Some(KeyCode::Key3) => Some(Command::DecreaseCycle),
            Some(KeyCode::Key4) => Some(Command::DecreaseAngleSpeed),
            Some(KeyCode::Key5) => Some(Command::SetZeroAngleSpeed),
            Some(KeyCode::Key6) => Some(Command::IncreaseAngleSpeed),
            Some(KeyCode::Key7) => Some(Command::CameraUp),
            Some(KeyCode::Key8) => Some(Command::CameraReset),
            Some(KeyCode::Key9) => Some(Command::CameraDown),
            Some(KeyCode::D) => Some(Command::NextMesh),
            Some(KeyCode::H) => Some(Command::NextVertexShader),
            Some(KeyCode::L) => Some(Command::NextFragmentShader),
            Some(KeyCode::C) => Some(Command::VShader("default".to_string())),
            Some(KeyCode::G) => Some(Command::VShader("inner_rotation".to_string())),
            Some(KeyCode::B) => Some(Command::FShader("default".to_string())),
            Some(KeyCode::F) => Some(Command::FShader("test".to_string())),
            Some(KeyCode::J) => Some(Command::FShader("rgb_corners".to_string())),
            Some(KeyCode::A) => Some(Command::FShader("corners".to_string())),
            Some(KeyCode::E) => Some(Command::FShader("time".to_string())),
            Some(KeyCode::I) => Some(Command::FShader("rainbow".to_string())),
            _ => None,
        }
    }
}
