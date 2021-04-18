use dxlib::{
    application::Application, dx3d::model::Mv1Model, math::vector::Vector3, utils::wait_any_key,
};
use dxlib_sys::dx_SetFullSceneAntiAliasingMode;

fn main() -> anyhow::Result<()> {
    unsafe { dx_SetFullSceneAntiAliasingMode(4, 2) };
    
    let app = Application::builder().screen_size(640, 1000).build()?;

    let mut model = Mv1Model::load("./examples/resources/model/unitychan.mv1")?;

    model.set_position(Vector3::from([320.0, -340.0, 1000.0]))?;

    model.draw()?;

    app.screenshot(None, "screenshot.bmp")?;

    wait_any_key()?;

    Ok(())
}
