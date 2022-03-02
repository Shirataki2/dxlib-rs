use crate::components as cmp;
use crate::resources as res;
use dxlib::prelude::*;
use rand::Rng;
use specs::*;
use std::fmt::Write as _;

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
                }
                .draw();
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
        Write<'a, res::GameState>,
        Write<'a, res::PlayerData>,
        WriteStorage<'a, cmp::Position>,
        WriteStorage<'a, cmp::Velocity>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(
        &mut self,
        (setting, mut state, mut data, mut pos, mut vel, obj_type): Self::SystemData,
    ) {
        for (pos, vel_, obj_type) in (&mut pos, &mut vel, &obj_type).join() {
            if *obj_type == cmp::ObjectType::Ball {
                let vel: Vector2<f32> = vel_.0;
                let vel = vel.normalized() * setting.ball_velocity;
                pos.0 += vel;
                if pos[1] > 1.1 {
                    if data.lives > 0 {
                        data.lives -= 1;
                        (*pos).0 = Vector2::from([-1.0, -1.0]);
                        (*vel_).0 = Vector2::from([0.0, 0.0]);
                        *state = res::GameState::Restart;
                    } else {
                        *state = res::GameState::GameOver;
                    }
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
                }
                .draw();
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
        let mut rng = rand::thread_rng();
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
                    if ball_left < pad_right
                        && ball_right > pad_left
                        && ball_top < pad_bottom
                        && ball_bottom > pad_top
                    {
                        let coef = if KeyBoard::is_hit(Key::LEFT) {
                            rng.gen_range(0.90..=0.97)
                        } else if KeyBoard::is_hit(Key::RIGHT) {
                            rng.gen_range(1.03..=1.1)
                        } else {
                            rng.gen_range(0.95..=1.05)
                        };
                        ball_vel[1] *= -coef;
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
                        brick_left,
                        brick_right,
                        brick_top,
                        brick_bottom,
                        ball_left,
                        ball_right,
                        ball_top,
                        ball_bottom,
                    );
                    match hit {
                        HitCheck::X => {
                            ball_vel[0] *= -rng.gen_range(0.98..=1.02);
                        }
                        HitCheck::Y => {
                            ball_vel[1] *= -rng.gen_range(0.98..=1.02);
                        }
                        HitCheck::None => {}
                        HitCheck::Both => {
                            ball_vel[0] *= -rng.gen_range(0.98..=1.02);
                            ball_vel[1] *= -rng.gen_range(0.98..=1.02);
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
#[allow(unused_variables)]
fn hit_check(al: f32, ar: f32, at: f32, ab: f32, bl: f32, br: f32, bt: f32, bb: f32) -> HitCheck {
    let bxc = (bl + br) / 2.0;
    let byc = (bt + bb) / 2.0;
    let top = bxc < ar && bxc > al && bb > at && bb < ab;
    let bottom = bxc < ar && bxc > al && bt > at && bt < ab;
    let left = bl < al && br > al && byc > at && byc < ab;
    let right = bl < ar && br > ar && byc > at && byc < ab;
    let xside = left || right;
    let yside = top || bottom;
    match (xside, yside) {
        (true, true) => HitCheck::Both,
        (true, false) => HitCheck::X,
        (false, true) => HitCheck::Y,
        (false, false) => HitCheck::None,
    }
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
                }
                .draw();
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
        let _ = writeln!(writer.0, "FPS: {:0.1}", fps);
    }
}

pub struct UiRenderer;

impl<'a> System<'a> for UiRenderer {
    type SystemData = (
        Read<'a, res::GameState>,
        Read<'a, res::ScreenSize>,
        Read<'a, res::PlayerData>,
        ReadStorage<'a, cmp::Position>,
        ReadStorage<'a, cmp::ObjectType>,
    );

    fn run(&mut self, (state, size, player_data, pos, obj_type): Self::SystemData) {
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
                let style = builder.coord(pos).font(&font).color(Color::black()).build();
                let res = style.draw(&format!("SCORE: {}\nLIVES: {}", score, lives));
                if res.is_err() {
                    error!("Failed to draw ui: {:?}", res);
                }
                let pos = Vector2::from([(size.width as f32 * 0.1) as i32, size.height as i32 / 2]);
                let style = builder.coord(pos).font(&font).color(Color::blue()).build();
                let _ = match *state {
                    res::GameState::Restart => style.draw("Press 'Space' to start again"),
                    res::GameState::GameOver => style.draw("Press 'R' to start new game"),
                    _ => Ok(()),
                };
            }
        }
    }
}

pub struct KeyInteraction;
impl<'a> System<'a> for KeyInteraction {
    type SystemData = (
        Write<'a, res::GameState>,
        Write<'a, res::PlayerData>,
        Write<'a, res::PlayerSettings>,
        WriteStorage<'a, cmp::Position>,
        WriteStorage<'a, cmp::Velocity>,
        WriteStorage<'a, cmp::ObjectType>,
    );

    fn run(
        &mut self,
        (mut state, mut data, mut setting, mut pos, mut vel, mut obj_type): Self::SystemData,
    ) {
        if *state == res::GameState::Restart {
            for (pos, vel, obj_type) in (&mut pos, &mut vel, &mut obj_type).join() {
                if *obj_type == cmp::ObjectType::Ball && KeyBoard::is_hit(Key::SPACE) {
                    *state = res::GameState::Running;
                    let mut rng = rand::thread_rng();
                    let sign: i32 = rng.gen_range(0..=1) * 2 - 1;
                    (*pos).0 = Vector2::from([0.5, 0.5]);
                    (*vel).0 = Vector2::from([
                        rng.gen_range(0.6..=1.4) * sign as f32,
                        rng.gen_range(0.6..=1.4),
                    ]);
                }
            }
        }

        if *state == res::GameState::GameOver && KeyBoard::is_hit(Key::R) {
            for obj_type in (&mut obj_type).join() {
                if *obj_type == cmp::ObjectType::Empty {
                    *obj_type = cmp::ObjectType::Brick;
                }
            }
            for (pos, vel, obj_type) in (&mut pos, &mut vel, &mut obj_type).join() {
                if *obj_type == cmp::ObjectType::Ball {
                    *data = res::PlayerData::default();
                    *state = res::GameState::Running;
                    let mut rng = rand::thread_rng();
                    let sign: i32 = rng.gen_range(0..=1) * 2 - 1;
                    (*pos).0 = Vector2::from([0.5, 0.5]);
                    (*vel).0 = Vector2::from([
                        rng.gen_range(0.6..=1.4) * sign as f32,
                        rng.gen_range(0.6..=1.4),
                    ]);
                }
            }
        }

        if *state == res::GameState::Running {
            let def = res::PlayerSettings::default();
            setting.ball_velocity = if KeyBoard::is_hit(Key::Q) {
                def.ball_velocity * 2.0
            } else {
                def.ball_velocity
            };
            setting.paddle_velocity = if KeyBoard::is_hit(Key::E) {
                def.paddle_velocity * 2.0
            } else {
                def.paddle_velocity
            };
        }
    }
}
