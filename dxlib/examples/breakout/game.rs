use crate::components as cmp;
use crate::entities as ent;
use crate::resources as res;
use crate::systems as sys;
use dxlib::prelude::*;
use specs::*;

pub struct Breakout {
    app: Application,
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl Breakout {
    pub fn new(width: usize, height: usize) -> anyhow::Result<Breakout> {
        let mut app = Application::builder()
            .screen_size(width, height)
            .title("ECSモデルを利用したブロック崩し")
            .add_plugin(BackgroundPlugin {
                run_always: true,
                color: Color::white(),
            })?
            .build()?;
        app.screen.set_draw_screen(DrawScreen::Back)?;

        let mut world = World::new();

        world.register::<cmp::Position>();
        world.register::<cmp::Velocity>();
        world.register::<cmp::Rect>();
        world.register::<cmp::Color>();
        world.register::<cmp::ObjectType>();

        world.insert(res::PlayerSettings::default());
        world.insert(res::Writer(DebugWriter::default()));
        world.insert(res::FpsRecorder(Some(Fps::new(240))));

        world.insert(res::ScreenSize { width, height });
        world.insert(res::PlayerData::default());
        world.insert(res::GameState::default());

        ent::Paddle::register(
            &mut world,
            Vector2::from([0.5, 0.95]),
            Vector2::from([0.125, 0.03]),
            Color::red(),
        );

        ent::Ball::register(
            &mut world,
            Vector2::from([0.5, 0.5]),
            Vector2::from([0.25, 0.25]),
            Vector2::from([0.0125, 0.0125]),
            Color::blue(),
        );

        ent::Ui::register(&mut world, Vector2::from([0.4, 0.0]));

        let brick_size = Vector2::from([0.09, 0.04]);
        let brick_colors = vec![
            Color::<u8>::green(),
            Color::blue(),
            Color::magenta(),
            Color::cyan(),
        ];

        for y in 0..6 {
            for x in 0..8 {
                let position = Vector2::from([
                    0.5 - 0.10 * (x as f32 - 3.5),
                    0.25 - 0.05 * (y as f32 - 3.0),
                ]);
                let color = brick_colors[y % brick_colors.len()];
                ent::Brick::register(&mut world, position, brick_size, color);
            }
        }

        let dispatcher = DispatcherBuilder::new()
            .with(sys::ClearDebugLog, "clear_debug_log", &[])
            .with(sys::BallCollider, "ball_collider", &[])
            .with(sys::PaddleMover, "paddle_mover", &[])
            .with(sys::BallMover, "ball_mover", &["ball_collider"])
            .with(sys::PaddleRenderer, "paddle_renderer", &[])
            .with(sys::BallRenderer, "ball_renderer", &[])
            .with(sys::BrickRenderer, "bricks_renderer", &[])
            .with(sys::UiRenderer, "ui_renderer", &[])
            .with(sys::KeyInteraction, "key_interaction", &[])
            .with(sys::ShowFps, "show_fps", &["clear_debug_log"])
            .build();

        Ok(Breakout {
            app,
            world,
            dispatcher,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while self.app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
            self.app.screen.clear()?;

            self.dispatcher.dispatch(&self.world);
            self.world.maintain();

            self.app.screen.flip()?;
        }
        Ok(())
    }
}
