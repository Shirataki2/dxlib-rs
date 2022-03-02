#[macro_use]
extern crate log;

mod components;
mod entities;
mod game;
mod resources;
mod systems;

fn main() -> anyhow::Result<()> {
    load_font()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let mut game = game::Breakout::new(640, 480)?;
    game.run()?;
    unload_font()?;
    Ok(())
}

fn load_font() -> anyhow::Result<()> {
    use std::{ffi::CString, ptr::null_mut};
    use winapi::um::wingdi::AddFontResourceExA;
    let font_path = CString::new("../resources/fonts/MplusCodeLatin60-Bold.ttf")?;
    let code = unsafe { AddFontResourceExA(font_path.as_ptr(), 0, null_mut()) };
    if code != 0 {
        panic!("Failed to load font");
    }
    Ok(())
}

fn unload_font() -> anyhow::Result<()> {
    use std::{ffi::CString, ptr::null_mut};
    use winapi::um::wingdi::RemoveFontResourceExA;
    let font_path = CString::new("../resources/fonts/MplusCodeLatin60-Bold.ttf")?;
    unsafe { RemoveFontResourceExA(font_path.as_ptr(), 0, null_mut()) };
    Ok(())
}
