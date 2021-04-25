use std::{fmt::Write, time::Duration};

use dxlib::{
    graphics::live2d::{Live2DModel, Live2DRenderer},
    prelude::*,
};
use rand::{thread_rng, Rng};

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder()
        .screen_size(380, 740)
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })?
        .title("Live2Dモデルの表示テスト")
        .build()?;

    let mut rng = thread_rng();

    let mut writer = DebugWriter::default();

    let mut renderer = Live2DRenderer::new("Live2DCubismCore.dll")?;
    let model = Live2DModel::load("./resources/Hiyori/Hiyori.model3.json")?;
    renderer.add_model(&model)?;
    model.scale(Vector2::from([4.0, 4.0]))?;

    app.screen.set_draw_screen(DrawScreen::Back)?;

    let mut fps = Fps::new(60);
    while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        app.screen.clear()?;
        writer.clear()?;

        let mouse_pos = Mouse::get_position_as_f32()?;
        if model.is_motion_finished()? {
            model.start_motion("Idle", rng.gen_range(0..=8))?;
        }
        model.update(Duration::from_secs_f32(1.0 / 60.0))?;
        renderer.render()?;
        let hits = model.list_hits(&mouse_pos)?;
        writeln!(writer, "FPS   : {:.1}", fps.update())?;
        writeln!(writer, "Mouse : {:?}", mouse_pos)?;
        writeln!(writer, "Hits  : {:?}", hits)?;
        app.screen.flip()?;
    }

    Ok(())
}
