use std::fmt::Write as _;
use specs::*;
use dxlib::prelude::*;
use crate::components as cmp;
use crate::resources as res;

pub struct PaddleMover;

impl<'a> System<'a> for PaddleMover {
    type SystemData = (
        Read<'a, res::PlayerSettings>,
        WriteStorage<'a, cmp::Position>,
        ReadStorage<'a, cmp::Rect>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(&mut self, (setting, mut pos, rect, obj_type): Self::SystemData) {
        for (pos, rect, obj_type) in (&mut pos, &rect, &obj_type).join() {
            if *obj_type == cmp::ObjectType::Paddle {
                let v = setting.paddle_velocity;
                if KeyBoard::is_hit(Key::LEFT) {
                    pos[0] = (pos[0] - v).max(rect[0] / 2.0);
                }
                if KeyBoard::is_hit(Key::RIGHT) {
                    pos[0] = (pos[0] + v).min(1.0 - rect[0] / 2.0);
                }
            }
        }
    }

}

pub struct PaddleRenderer;

impl<'a> System<'a> for PaddleRenderer {
    type SystemData = (
        Read<'a, res::ScreenSize>,
        ReadStorage<'a, cmp::Rect>,
        ReadStorage<'a, cmp::Position>,
        ReadStorage<'a, cmp::Color>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(&mut self, (size, rect, pos, color, obj_type): Self::SystemData) {
        for (rect, pos, color, obj_type) in (&rect, &pos, &color, &obj_type).join() {
            if *obj_type == cmp::ObjectType::Paddle {
                let rect = size.to_absolute_position(rect.0);
                let pos = size.to_absolute_position(pos.0);
                let color = color.0;
                let res = Rect {
                    left_top: Vector2::from([
                        (pos[0] - rect[0] / 2.0) as i32,
                        (pos[1] - rect[1] / 2.0) as i32,
                    ]),
                    right_bottom: Vector2::from([
                        (pos[0] + rect[0] / 2.0) as i32,
                        (pos[1] + rect[1] / 2.0) as i32,
                    ]),
                    filled: true,
                    color,
                }.draw();
                if res.is_err() {
                    error!("Failed to draw paddle: {:?}", res);
                }
            }
        }
    }
}

pub struct BallRenderer;

impl<'a> System<'a> for BallRenderer {
    type SystemData = (
        Read<'a, res::ScreenSize>,
        ReadStorage<'a, cmp::Rect>,
        ReadStorage<'a, cmp::Position>,
        ReadStorage<'a, cmp::Color>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(&mut self, (size, rect, pos, color, obj_type): Self::SystemData) {
        for (rect, pos, color, obj_type) in (&rect, &pos, &color, &obj_type).join() {
            if *obj_type == cmp::ObjectType::Ball {
                let rect = size.to_absolute_position(rect.0);
                let pos = size.to_absolute_position(pos.0);
                let color = color.0;
                let res = Circle {
                    center: Vector2::from([pos[0] as i32, pos[1] as i32]),
                    radius: rect[0] as i32 / 2,
                    color,
                    filled: true,
                    thickness: 1,
                }.draw();
                if res.is_err() {
                    error!("Failed to draw ball: {:?}", res);
                }
            }
        }
    }
}

pub struct ClearDebugLog;

impl<'a> System<'a> for ClearDebugLog {
    type SystemData = Write<'a, res::Writer>;
    
    fn run(&mut self, mut writer: Self::SystemData) {
        let _ = writer.0.clear();
    }
}

pub struct ShowFps;

impl<'a> System<'a> for ShowFps {
    type SystemData = (Write<'a, res::FpsRecorder>, Write<'a, res::Writer>);
    
    fn run(&mut self, (mut fps, mut writer): Self::SystemData) {
        let fps = fps.0.as_mut().unwrap().update();
        let _ = writeln!(writer.0, "FPS: {:0.2}", fps);
    }
}
