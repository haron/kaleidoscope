use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, Default, Debug)]
#[serde(rename_all = "lowercase")]
pub enum KeyboardType {
    #[default]
    Full,
    Mini,
}

pub enum Command {
    Quit,
    NextMesh,
    CameraUp,
    CameraDown,
    // Height change in knob detents, fractional for smooth zooms
    CameraShift(f32),
    CameraReset,
    SwitchRotation,
    NextFragmentShader,
    PrevFragmentShader,
    NextVertexShader,
    PrevVertexShader,
    FShader(String),
    VShader(String),
    Shaders(String, String),
    IncreaseAngleSpeed,
    DecreaseAngleSpeed,
    SetZeroAngleSpeed,
    IncreaseCycle,
    DecreaseCycle,
    ResetCycle,
}
