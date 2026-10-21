use serde::Deserialize;

use crate::controls::KeyboardType;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct WebCameraSettings {
    pub width: u32,
    pub height: u32,
    pub device_id: usize,
    pub saturation: i64,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Camera3DSettings {
    pub min_height: f32,
    pub max_height: f32,
    pub start_height: f32,
    pub step: f32,
    pub angle_step: f32,
    pub max_angle_step: f32,
    pub angle_speed_change_step: f32,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct AutopilotSettings {
    pub idle: f64,
    pub period: f64,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Trianges {
    pub levels: i16,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Hexagons {
    pub levels: i16,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Mesh {
    pub triangles: Trianges,
    pub hexagons: Hexagons,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Settings {
    pub keyboard: KeyboardType,
    pub webcamera: WebCameraSettings,
    pub camera3d: Camera3DSettings,
    pub autopilot: AutopilotSettings,
    pub mesh: Mesh,
    pub default_cycle: u32,
    pub max_cycle: u32,
    pub cycle_mult: f32,
    pub crossfade: f64,
    pub fullscreen: bool,
    pub window_height: i32,
    pub window_width: i32,
}
