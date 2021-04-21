use dxlib::{
    dx3d::prelude::*,
    ext::{debug::camera::MouseCamera, models::GroundGrid},
    plugin::FullSceneAntiAliasPlugin,
    prelude::*,
};
use dxlib_sys::*;
use std::fmt::Write;

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder()
        .screen_size(1000, 1000)
        .title("3Dモデルの表示テスト")
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })?
        .add_plugin(FullSceneAntiAliasPlugin {
            samples: 4,
            quality: 2,
        })?
        .build()?;

    app.screen.set_draw_screen(DrawScreen::Back)?;

    let mut writer = DebugWriter::default();
    let mut camera = Camera::default();
    let grid = GroundGrid::default();

    unsafe {
        dx_MV1SetLoadModelUsePhysicsMode(DX_LOADMODEL_PHYSICS_REALTIME);
    }
    camera.set_near_far(1.0, 100.0)?;

    let mut model = Mv1Model::load("./examples/resources/model/Lat式ミクVer2.3_Normal.pmd")?;

    let mut camera = MouseCamera::new(
        camera,
        Vector3::from([0.0, 16.0, -20.0]),
        Vector3::from([0.0, 11.0, 0.0]),
        4e-3,
        1.7,
        2.0,
    );
    Fog::enable(true)?;
    Fog::set_color(Color::blue())?;
    Fog::set_fog_start_end(100.0, 1000.0)?;

    World::set_ambient(Color::blue())?;
    unsafe {
        dx_MV1PhysicsResetState(model.handle);
        dx_MV1SetShapeRate(model.handle, 29, 1.0, 0);
        let anim_id = 4;
        let idx = dx_MV1AttachAnim(model.handle, anim_id, -1, 0);
        let total_time = dx_MV1GetAnimTotalTime(model.handle, anim_id);
        let mut t = 0.0;
        let mut ctr: i128 = 0;
        let mut fps = Fps::new(240);
        while app.update().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
            writer.clear()?;
            camera.update()?;

            grid.draw()?;

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
                _ => 0.0,
            };
            dx_MV1SetShapeRate(model.handle, 16, r, 0);

            model.draw()?;

            writeln!(writer, "FPS: {:.2}", fps.update())?;
            writeln!(writer, "{:?}", camera)?;
            ctr += 1;
        }
    }

    Ok(())
}
