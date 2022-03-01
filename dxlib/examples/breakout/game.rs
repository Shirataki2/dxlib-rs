use specs::*;
use dxlib::prelude::*;
use crate::systems as sys;
use crate::resources as res;
use crate::components as cmp;
use crate::entities as ent;

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
        world.register::<cmp::Rect>();
        world.register::<cmp::Color>();
        world.register::<cmp::ObjectType>();

        world.insert(res::PlayerSettings::default());
        world.insert(res::Writer(DebugWriter::default()));
        world.insert(res::FpsRecorder(Some(Fps::new(240))));
    
        world.insert(res::ScreenSize { width, height });

        ent::Paddle::register(
            &mut world, 
            Vector2::from([0.5, 0.95]),
            Vector2::from([0.125, 0.03]), 
            Color::red()
        );

        ent::Ball::register(
            &mut world, 
            Vector2::from([0.5, 0.5]),
            Vector2::from([0.02, 0.02]), 
            Color::blue()
        );

        let dispatcher = DispatcherBuilder::new()
            .with(sys::ClearDebugLog, "clear_debug_log", &[])
            .with(sys::PaddleMover, "paddle_mover", &[])
            .with(sys::BallRenderer, "ball_renderer", &[])
            .with(sys::PaddleRenderer, "paddle_renderer", &[])
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