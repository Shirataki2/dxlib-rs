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

pub struct BallMover;
impl<'a> System<'a> for BallMover {
    type SystemData = (
        Read<'a, res::PlayerSettings>,
        WriteStorage<'a, cmp::Position>,
        ReadStorage<'a, cmp::Velocity>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(&mut self, (setting, mut pos, vel, obj_type): Self::SystemData) {
        for (pos, vel, obj_type) in (&mut pos, &vel, &obj_type).join() {
            if *obj_type == cmp::ObjectType::Ball {
                let vel: Vector2<f32> = vel.0;
                let vel = vel.normalized() * setting.ball_velocity;
                pos.0 += vel;
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

pub struct BallCollider;

impl<'a> System<'a> for BallCollider {
    type SystemData = (
        Write<'a, res::PlayerData>,
        WriteStorage<'a, cmp::ObjectType>,
        ReadStorage<'a, cmp::Rect>,
        ReadStorage<'a, cmp::Position>,
        WriteStorage<'a, cmp::Velocity>,
    );

    fn run(&mut self, (mut data, mut obj_type, rect, pos, mut vel): Self::SystemData) {
        let mut ball = None;
        // Wall Collision
        for (pos, vel, rect, obj_type) in (&pos, &mut vel, &rect, &obj_type).join() {
            if *obj_type == cmp::ObjectType::Ball {
                let pos: Vector2<f32> = pos.0;
                let r: Vector2<f32> = rect.0;
                if pos[0] < r[0] || pos[0] > 1.0 - r[0] {
                    vel.0[0] = -vel.0[0];
                }
                if pos[1] < r[0] {
                    vel.0[1] = -vel.0[1];
                }
                ball = Some((pos, vel, r))
            }
        }
        if let Some((ball_pos, ball_vel, ball_r)) = ball {
            for (pos, rect, obj_type) in (&pos, &rect, &mut obj_type).join() {
                // Paddle Collision
                if *obj_type == cmp::ObjectType::Paddle {
                    let pad_pos: Vector2<f32> = pos.0;
                    let pad_rect: Vector2<f32> = rect.0;
                    let pad_left = pad_pos[0] - pad_rect[0] / 2.0;
                    let pad_right = pad_pos[0] + pad_rect[0] / 2.0;
                    let pad_top = pad_pos[1] - pad_rect[1] / 2.0;
                    let pad_bottom = pad_pos[1] - pad_rect[1] / 2.0;
                    let ball_left = ball_pos[0] - ball_r[0] / 2.0;
                    let ball_right = ball_pos[0] + ball_r[0] / 2.0;
                    let ball_top = ball_pos[1] - ball_r[1] / 2.0;
                    let ball_bottom = ball_pos[1] + ball_r[1] / 2.0;
                    if ball_left < pad_right && ball_right > pad_left && ball_top < pad_bottom && ball_bottom > pad_top {
                        ball_vel[1] = -ball_vel[1];
                    }  
                }
                // Brick Collision
                if *obj_type == cmp::ObjectType::Brick {
                    let brick_pos: Vector2<f32> = pos.0;
                    let brick_rect: Vector2<f32> = rect.0;
                    let brick_left = brick_pos[0] - brick_rect[0] / 2.0;
                    let brick_right = brick_pos[0] + brick_rect[0] / 2.0;
                    let brick_top = brick_pos[1] - brick_rect[1] / 2.0;
                    let brick_bottom = brick_pos[1] + brick_rect[1] / 2.0;
                    let ball_left = ball_pos[0] - ball_r[0] / 2.0;
                    let ball_right = ball_pos[0] + ball_r[0] / 2.0;
                    let ball_top = ball_pos[1] - ball_r[1] / 2.0;
                    let ball_bottom = ball_pos[1] + ball_r[1] / 2.0;
                    let hit = hit_check(
                        brick_left, brick_right, brick_top, brick_bottom, 
                        ball_left, ball_right, ball_top, ball_bottom
                    );
                    match hit {
                        HitCheck::X => {
                            ball_vel[0] = -ball_vel[0];
                        }
                        HitCheck::Y => {
                            ball_vel[1] = -ball_vel[1];
                        }
                        HitCheck::None => {}
                        HitCheck::Both => {
                            ball_vel[0] = -ball_vel[0];
                            ball_vel[1] = -ball_vel[1];
                        }
                    }
                    if !matches!(hit, HitCheck::None) {
                        *obj_type = cmp::ObjectType::Empty;
                        data.score += 100;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum HitCheck {
    None,
    X,
    Y,
    Both,
}

#[allow(clippy::too_many_arguments)]
fn hit_check(
    al: f32, ar: f32, at: f32, ab: f32,
    bl: f32, br: f32, bt: f32, bb: f32,
) -> HitCheck {
    let xside = (al < br) && (ar > bl);
    let yside = (at < bb) && (ab > bt);
    if !(xside && yside) {
        return HitCheck::None;
    }
    if xside && !yside {
        return HitCheck::X;
    }
    if yside {
        return HitCheck::Y;
    }
    HitCheck::Both
}
        


pub struct BrickRenderer;

impl<'a> System<'a> for BrickRenderer {
    type SystemData = (
        Read<'a, res::ScreenSize>,
        ReadStorage<'a, cmp::Rect>,
        ReadStorage<'a, cmp::Position>,
        ReadStorage<'a, cmp::Color>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(&mut self, (size, rect, pos, color, obj_type): Self::SystemData) {
        for (rect, pos, color, obj_type) in (&rect, &pos, &color, &obj_type).join() {
            if *obj_type == cmp::ObjectType::Brick {
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
                    error!("Failed to draw brick: {:?}", res);
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

pub struct UiRenderer;

impl<'a> System<'a> for UiRenderer {
    type SystemData = (
        Read<'a, res::ScreenSize>,
        Read<'a, res::PlayerData>,
        ReadStorage<'a, cmp::Position>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(&mut self, (size, player_data, pos, obj_type): Self::SystemData) {
        let score = player_data.score;
        let lives = player_data.lives;
        for (pos, obj_type) in (&pos, &obj_type).join() {
            if *obj_type == cmp::ObjectType::UI {
                let pos = size.to_absolute_position(pos.0);
                let pos = Vector2::from([pos[0] as i32, pos[1] as i32]);
                let font = match Font::builder()
                    .name("M PLUS Code Latin 60")
                    .font_type(FontType::AntiAliasing4x4)
                    .size(20)
                    .thickness(3)
                    .build()
                {
                    Ok(font) => font,
                    Err(err) => {
                        error!("Failed to build font: {:?}", err);
                        return;
                    }
                };
                let mut builder = TextStyle::builder();
                let style = builder
                    .coord(pos)
                    .font(&font)
                    .color(Color::black())
                    .build();
                let res = style.draw(&format!("SCORE: {}\nLIVES: {}", score, lives));
                if res.is_err() {
                    error!("Failed to draw ui: {:?}", res);
                }
            }
        }
    }
}