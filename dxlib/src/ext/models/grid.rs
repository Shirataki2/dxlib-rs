use crate::{color::Color, dx3d::draw::Line3D, error::Result, math::vector::Vector3};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault)]
pub struct GroundGrid {
    #[default = 50]
    pub num: usize,
    #[default = 10.0]
    pub interval: f32,
    #[default = 200.0]
    pub line_length: f32,
}

impl GroundGrid {
    pub fn draw(&self) -> Result<()> {
        let l = self.line_length;
        Line3D {
            start: Vector3::from([-l, 0.0, 0.0]),
            end: Vector3::from([l, 0.0, 0.0]),
            color: Color::red(),
        }
        .draw()?;
        Line3D {
            start: Vector3::from([0.0, 0.0, l]),
            end: Vector3::from([0.0, 0.0, -l]),
            color: Color::red(),
        }
        .draw()?;
        for i in 1..=self.num {
            Line3D {
                start: Vector3::from([-l, 0.0, self.interval * i as f32]),
                end: Vector3::from([l, 0.0, self.interval * i as f32]),
                color: Color::black(),
            }
            .draw()?;
            Line3D {
                start: Vector3::from([self.interval * i as f32, 0.0, -l]),
                end: Vector3::from([self.interval * i as f32, 0.0, l]),
                color: Color::black(),
            }
            .draw()?;
            Line3D {
                start: Vector3::from([-l, 0.0, -self.interval * i as f32]),
                end: Vector3::from([l, 0.0, -self.interval * i as f32]),
                color: Color::black(),
            }
            .draw()?;
            Line3D {
                start: Vector3::from([-self.interval * i as f32, 0.0, -l]),
                end: Vector3::from([-self.interval * i as f32, 0.0, l]),
                color: Color::black(),
            }
            .draw()?;
        }
        Ok(())
    }
}
