use std::time::Duration;

use dxlib::{
    application::Application,
    live2d::{Live2DModel, Live2DRenderer},
    screen::DrawScreen,
    vector::Vector2,
};
use rand::{thread_rng, Rng};

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder().screen_size(380, 740).build()?;

    let mut rng = thread_rng();

    let mut renderer = Live2DRenderer::new("Live2DCubismCore.dll")?;
    let model = Live2DModel::load("./examples/resources/Hiyori/Hiyori.model3.json")?;
    renderer.add_model(&model)?;
    model.scale(Vector2::from([4.0, 4.0]))?;

    app.screen.set_draw_screen(DrawScreen::Back)?;

    while app.process_message().is_ok() {
        app.screen.clear()?;
        if model.is_motion_finished()? {
            model.start_motion("Idle", rng.gen_range(0..=8))?;
        }
        model.update(Duration::from_millis(25))?;
        renderer.render()?;
        app.screen.flip()?;
    }

    Ok(())
}
