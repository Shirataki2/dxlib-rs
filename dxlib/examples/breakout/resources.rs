use dxlib::prelude::*;

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

#[derive(Debug)]
pub struct PlayerSettings {
    pub paddle_velocity: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            paddle_velocity: 0.0125,
        }
    }
}