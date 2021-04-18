use std::{fmt::Write, thread, time::Duration};

use dxlib::{
    application::Application,
    input::{
        keyboard::{Key, KeyBoard},
        mouse::Mouse,
    },
    screen::DrawScreen,
    writer::DebugWriter,
};

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder().screen_size(800, 400).build()?;
    let mut writer = DebugWriter::default();

    app.screen.set_draw_screen(DrawScreen::Back)?;

    while app.process_message().is_ok() && !KeyBoard::is_hit(Key::ESCAPE) {
        app.screen.clear()?;
        writeln!(
            writer,
            "{:?} {:?}",
            KeyBoard::list_hit_keys()?,
            Mouse::get_input_log()
        )?;
        thread::sleep(Duration::from_millis(50));
        app.screen.flip()?;
    }

    Ok(())
}
