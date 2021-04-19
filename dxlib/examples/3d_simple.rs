use dxlib::{
    application::Application,
    color::Color,
    dx3d::model::Mv1Model,
    ext::debug::fps::Fps,
    input::keyboard::{Key, KeyBoard},
    math::vector::Vector3,
    plugin::BackgroundPlugin,
    screen::DrawScreen,
    writer::DebugWriter,
};
use dxlib_sys::*;
use std::fmt::Write;

fn main() -> anyhow::Result<()> {
    unsafe { dx_SetFullSceneAntiAliasingMode(4, 2) };

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
        dx_MV1SetLoadModelUsePhysicsMode(DX_LOADMODEL_PHYSICS_REALTIME);
        dx_SetCameraNearFar(0.1, 1000.0);
    }

    let mut model = Mv1Model::load("./examples/resources/model/Lat式ミクVer2.3_Normal.pmd")?;

    model.set_position(Vector3::from([0.0, 0.0, 0.0]))?;

    unsafe {
        dx_MV1PhysicsResetState(model.handle);
        dx_SetCameraPositionAndTarget_UpVecY(
            Vector3::from([0.0, 16.0, -20.0]).into(),
            Vector3::from([0.0, 11.0, 0.0]).into(),
        );
        dx_MV1SetShapeRate(model.handle, 29, 1.0, 0);
        let anim_id = 0;
        let idx = dx_MV1AttachAnim(model.handle, anim_id, -1, 0);
        let total_time = dx_MV1GetAnimTotalTime(model.handle, anim_id);
        let mut t = 0.0;
        let mut ctr: i128 = 0;
        let mut fps = Fps::new(240);
        while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
            app.screen.flip()?;
            app.screen.clear()?;
            writer.clear()?;

            t += 0.5;
            if t >= total_time {
                t = 0.0;
            }
            dx_MV1SetAttachAnimTime(model.handle, idx, t);

            if ctr > 10 {
                dx_MV1PhysicsCalculation(model.handle, 1000.0 / 60.0);
            }

            let r = match ctr % 300 {
                103 | 221 | 277 => 0.2,
                104 | 222 | 278 => 1.0,
                105 | 223 | 279 => 0.8,
                106 | 224 | 280 => 0.2,
                _ => 0.0
            };
            dx_MV1SetShapeRate(model.handle, 16, r, 0);

            model.draw()?;

            writeln!(writer, "FPS: {:.2}", fps.update())?;
            ctr += 1;
        }
    }

    Ok(())
}
