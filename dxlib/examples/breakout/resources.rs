use dxlib::prelude::*;
use smart_default::SmartDefault;

#[derive(Debug, Default)]
pub struct Writer(pub DebugWriter);

#[derive(Default)]
pub struct FpsRecorder(pub Option<Fps>);

#[derive(Debug, Default)]
pub struct ScreenSize {
    pub width: usize,
    pub height: usize,
}

impl ScreenSize {
    pub fn to_absolute_position(&self, position: Vector2<f32>) -> Vector2<f32> {
        Vector2::from([
            position[0] * self.width as f32,
            position[1] * self.height as f32,
        ])
    }
}

#[derive(Debug, SmartDefault)]
pub struct PlayerSettings {
    #[default(0.0125)]
    pub paddle_velocity: f32,
    #[default(0.0075)]
    pub ball_velocity: f32,
}

#[derive(Debug, SmartDefault)]
pub struct PlayerData {
    pub score: u64,
    #[default(5)]
    pub lives: u64,
}
