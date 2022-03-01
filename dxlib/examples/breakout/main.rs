#[macro_use] extern crate log;

mod resources;
mod components;
mod entities;
mod game;
mod systems;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut game = game::Breakout::new(640, 480)?;
    game.run()?;
    Ok(())
}
