use rand::Rng;
use std::fmt::Write as _;

use dxlib::prelude::*;
use specs::*;

const SIZE: usize = 640;
const NUM_BALLS: usize = 1_000;

#[derive(Debug, Default)]
struct Writer(DebugWriter);

#[derive(Default)]
struct FpsRecorder(Option<Fps>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct ObjectColor(Color<u8>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x: f32,
    y: f32,
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * 0.05;
            pos.y += vel.y * 0.05;
        }
    }
}

struct ClearDebugLog;

impl<'a> System<'a> for ClearDebugLog {
    type SystemData = Write<'a, Writer>;

    fn run(&mut self, mut writer: Self::SystemData) {
        let _ = writer.0.clear();
    }
}

struct Renderer;
impl<'a> System<'a> for Renderer {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, ObjectColor>);

    fn run(&mut self, (pos, color): Self::SystemData) {
        for (pos, color) in (&pos, &color).join() {
            let _ = CircleAntiAlias {
                center: Vector2::from([pos.x, pos.y]),
                radius: 10.0,
                color: color.0,
                ..Default::default()
            }
            .draw();
        }
    }
}

struct Bouncer;
impl<'a> System<'a> for Bouncer {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (pos, mut vel): Self::SystemData) {
        for (pos, vel) in (&pos, &mut vel).join() {
            if pos.x > SIZE as f32 - 10.0 || pos.x < 10.0 {
                vel.x = -vel.x;
            }
            if pos.y > SIZE as f32 - 10.0 || pos.y < 10.0 {
                vel.y = -vel.y;
            }
        }
    }
}

struct ShowFps;

impl<'a> System<'a> for ShowFps {
    type SystemData = (Write<'a, FpsRecorder>, Write<'a, Writer>);

    fn run(&mut self, (mut fps, mut writer): Self::SystemData) {
        let fps = fps.0.as_mut().unwrap().update();
        let _ = writeln!(writer.0, "FPS: {:0.2}", fps);
    }
}

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder()
        .screen_size(SIZE, SIZE)
        .title("ECSモデルを利用した描画")
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })?
        .build()?;

    app.screen.set_draw_screen(DrawScreen::Back)?;

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<ObjectColor>();
    world.insert(Writer(DebugWriter::default()));
    world.insert(FpsRecorder(Some(Fps::new(240))));

    for _ in 0..NUM_BALLS {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(10.0..(SIZE as f32 - 10.0));
        let y = rng.gen_range(10.0..(SIZE as f32 - 10.0));
        let mut vel_x: f32 = rng.gen_range(-30.0..30.0);
        let mut vel_y = rng.gen_range(-30.0..30.0);
        if vel_x.hypot(vel_y) < 10.0 {
            vel_x *= 5.0;
            vel_y *= 5.0;
        }
        let color = Color::from(&[
            rng.gen_range(64..250),
            rng.gen_range(64..250),
            rng.gen_range(64..250),
        ]);
        world
            .create_entity()
            .with(Position { x, y })
            .with(Velocity { x: vel_x, y: vel_y })
            .with(ObjectColor(color))
            .build();
    }

    let mut dispatcher = DispatcherBuilder::new()
        .with(ClearDebugLog, "clear_debug_log", &[])
        .with(UpdatePos, "update_pos", &[])
        .with(Renderer, "renderer", &["update_pos"])
        .with(Bouncer, "bouncer", &["update_pos"])
        .with(ShowFps, "show_fps", &["clear_debug_log", "renderer"])
        .build();

    while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        app.screen.clear()?;

        dispatcher.dispatch(&world);
        world.maintain();

        app.screen.flip()?;
    }
    Ok(())
}
