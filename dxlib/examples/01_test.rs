use dxlib::application::Application;

fn main() -> anyhow::Result<()> {
    let _app = Application::builder().screen_size(640, 480).build()?;

    Ok(())
}
