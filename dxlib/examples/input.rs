use std::{fmt::Write, thread, time::Duration};

use dxlib::{
    application::Application,
    input::keyboard::{Key, KeyBoard},
    screen::DrawScreen,
    writer::DebugWriter,
};

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder().screen_size(640, 480).build()?;
    let mut writer = DebugWriter::default();

    app.screen.set_draw_screen(DrawScreen::Back)?;

    while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        app.screen.clear()?;
        writeln!(writer, "{:?}", KeyBoard::list_hit_keys()?)?;
        thread::sleep(Duration::from_millis(50));
        app.screen.flip()?;
    }

    Ok(())
}
