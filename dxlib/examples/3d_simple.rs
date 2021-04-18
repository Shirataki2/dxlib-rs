use dxlib::{
    application::Application,
    color::Color,
    dx3d::model::Mv1Model,
    input::keyboard::{Key, KeyBoard},
    math::vector::Vector3,
    plugin::BackgroundPlugin,
    screen::DrawScreen,
    writer::DebugWriter,
};
use dxlib_sys::*;
use std::{fmt::Write, time};

fn main() -> anyhow::Result<()> {
    unsafe { dx_SetFullSceneAntiAliasingMode(8, 4) };

    let mut app = Application::builder()
        .screen_size(640, 1000)
        .title("3Dモデルの表示テスト")
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })
        .build()?;

    app.screen.set_draw_screen(DrawScreen::Back)?;

    let mut writer = DebugWriter::default();

    unsafe {
        dx_MV1SetLoadModelUsePhysicsMode(DX_LOADMODEL_PHYSICS_LOADCALC);
        dx_SetCameraNearFar(0.1, 1000.0);
    }

    let mut model = Mv1Model::load("./examples/resources/model/Lat式ミクVer2.3_Normal.pmd")?;

    model.set_position(Vector3::from([0.0, 0.0, 0.0]))?;

    unsafe {
        dx_SetCameraPositionAndTarget_UpVecY(
            Vector3::from([0.0, 10.0, -25.0]).into(),
            Vector3::from([0.0, 10.0, 0.0]).into(),
        );
        let anim_id = 1;
        let idx = dx_MV1AttachAnim(model.handle, anim_id, -1, 0);
        let total_time = dx_MV1GetAnimTotalTime(model.handle, anim_id);
        let mut t = 0.0;
        let timer = time::Instant::now();
        let mut prev = 0;
        while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
            app.screen.flip()?;
            app.screen.clear()?;
            writer.clear()?;

            t += 1.0;
            if t >= total_time {
                t = 0.0;
            }
            dx_MV1SetAttachAnimTime(model.handle, idx, t);
            model.draw()?;

            let now = timer.elapsed().as_millis();
            writeln!(writer, "FPS: {}", 1000 / (now - prev))?;
            prev = now;
        }
    }

    Ok(())
}
