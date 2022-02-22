use dxlib::{
    dx3d::prelude::*,
    ext::{debug::camera::MouseCamera, models::GroundGrid},
    plugin::FullSceneAntiAliasPlugin,
    prelude::*,
};
use dxlib_sys::*;
// use winapi::um::processthreadsapi::{GetPriorityClass, SetPriorityClass ,GetCurrentProcess};
use std::fmt::Write;

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder()
        .screen_size(1000, 1000)
        .title("3Dモデルの表示テスト")
        .window_style(WindowStyle::NoTitleBarNoBorder)
        .direct3d(Direct3D::Dx11)
        .add_plugin(BackgroundPlugin {
            run_always: true,
            color: Color::white(),
        })?
        .add_plugin(FullSceneAntiAliasPlugin {
            samples: 8,
            quality: 4,
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
    camera.set_projection_type(ProjectionType::Perspective(PerspectiveProjection::new(
        *Angle::from_degrees(50.0),
    )))?;

    let mut model = Mv1Model::load("./resources/model/Lat式ミクVer2.31_Normal.pmd")?;
    model.set_position(Vector3::from([0.0, 0.0, 5.0]))?;
    let mut camera = MouseCamera::new(
        camera,
        Vector3::from([0.0, 16.0, -25.0]),
        Vector3::from([0.0, 11.0, 0.0]),
        4e-3,
        1.7,
        2.0,
    );

    World::set_ambient(Color::red())?;

    model.reset_physics()?;
    model.set_shape(29, 1.0)?;
    let anim = model.attach_animation(0)?;
    let total_time = anim.get_total_time()?;
    let mut t = 0.0;
    let mut ctr: i128 = 0;
    let mut fps = Fps::new(240);
    while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        app.screen.clear()?;

        let fps = fps.update();

        writer.clear()?;
        camera.update()?;
        grid.draw()?;

        t += 30.0 / fps as f32;
        if t >= total_time {
            t = 0.0;
        }
        anim.set_time(t)?;

        if ctr > 0 {
            model.simulate_millis(1000.0 / fps as f32)?;
        }

        let r = match ctr % 300 {
            103 | 221 | 277 => 0.2,
            104 | 222 | 278 => 1.0,
            105 | 223 | 279 => 0.8,
            106 | 224 | 280 => 0.2,
            _ => 0.0,
        };
        model.set_shape(16, r)?;

        model.draw()?;

        writeln!(writer, "FPS: {:.2}", fps)?;
        writeln!(writer, "左クリック　＋　ドラッグ: 回転")?;
        writeln!(writer, "中クリック　＋　ドラッグ: 移動")?;
        writeln!(writer, "マウスホイール　　　　　: 拡大縮小")?;
        ctr += 1;

        app.screen.flip()?;
    }

    Ok(())
}
