use crate::{dx3d::prelude::*, error::Result, prelude::*};

#[derive(Debug)]
pub struct MouseCamera {
    camera: Camera,
    position: Vector3<f32>,
    rotate_sensitivity: f32,
    wheel_sensitivity: f32,
    drag_sensitivity: f32,
    target: Vector3<f32>,
    prev_mouse: Option<Vector2<f32>>,
    delta_sum: Vector2<f32>,
    v_angle: Angle<f32>,
    h_angle: Angle<f32>,
}

impl MouseCamera {
    pub fn new(
        camera: Camera,
        initial_position: Vector3<f32>,
        target: Vector3<f32>,
        sensitivity: f32,
        wheel_sensitivity: f32,
        drag_sensitivity: f32,
    ) -> MouseCamera {
        let v = initial_position - target;
        let r = v.magnitude();
        let v_angle = Angle::from_degrees(90.0) - Angle::from_radians((v[1] / r).acos());
        let h_angle = Angle::from_radians(v[0].signum() * (v[2] / v[2].hypot(v[0])).acos());
        MouseCamera {
            camera,
            position: initial_position,
            target,
            rotate_sensitivity: sensitivity,
            wheel_sensitivity,
            drag_sensitivity,
            prev_mouse: None,
            delta_sum: Vector2::default(),
            v_angle,
            h_angle,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn update(&mut self) -> Result<()> {
        let prev_pos = match self.prev_mouse {
            None => {
                let pos = Mouse::get_position_as_f32()?;
                self.prev_mouse = Some(pos);
                self.camera
                    .set_position_from_look_and_upvec_y(self.position, self.target)?;
                return Ok(());
            }
            Some(pos) => pos,
        };
        let now_pos = Mouse::get_position_as_f32()?;

        let wheel = Mouse::get_wheel_input(true)?;
        let buttons = Mouse::get_button()?;
        if !buttons.contains(&MouseButton::Left)
            && !buttons.contains(&MouseButton::Middle)
            && wheel == 0
        {
            self.prev_mouse = Some(now_pos);
            return Ok(());
        }
        let delta = (now_pos - prev_pos) * self.rotate_sensitivity;
        self.delta_sum += delta;
        let mut dist = self.position.distance(self.target);
        if wheel != 0 {
            dist -= wheel as f32 * self.wheel_sensitivity;
        }

        if buttons.contains(&MouseButton::Middle) {
            let up = Vector3::up();
            let right = (self.position - self.target).cross(up).normalized();
            self.target += right * -delta[0] * self.drag_sensitivity;
            self.position += right * -delta[0] * self.drag_sensitivity;
            self.target += up * delta[1] * self.drag_sensitivity;
            self.position += up * delta[1] * self.drag_sensitivity;
            self.prev_mouse = Some(now_pos);
            self.camera
                .set_position_from_look_and_upvec_y(self.position, self.target)?;
            return Ok(());
        }

        self.v_angle -= Angle::from_radians(-delta[1]);
        self.h_angle += Angle::from_radians(delta[0]);
        let pos1 = Vector3::from([0.0, dist * self.v_angle.sin(), dist * -self.v_angle.cos()]);

        let pos2 = Vector3::from([
            self.h_angle.cos() * pos1[0] - self.h_angle.sin() * pos1[2],
            pos1[1],
            self.h_angle.sin() * pos1[0] - self.h_angle.cos() * pos1[2],
        ]);
        self.position = self.target + pos2;
        self.prev_mouse = Some(now_pos);
        self.camera
            .set_position_from_look_and_upvec_y(self.position, self.target)?;
        Ok(())
    }
}
