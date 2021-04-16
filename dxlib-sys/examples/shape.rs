use dxlib_sys::*;
use std::{process::exit, ptr};


fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE)?;        
        dx_SetGraphMode(640, 480, 32, 60)?;

        if dx_DxLib_Init()? != 0 {
            exit(1);
        }

        let white = dx_GetColor(255, 255, 255)?;
        let black = dx_GetColor(0, 0, 0)?;
        let red = dx_GetColor(255, 0, 0)?;
        let blue = dx_GetColor(0, 0, 255)?;
        let green = dx_GetColor(0, 255, 0)?;

        dx_DrawLine(0, 0, 640, 480, white, 2)?;
        dx_DrawLineAA(640.0, 0.0, 0.0, 480.0, red, 5.0)?;

        dx_DrawBox(100, 100, 540, 380, blue, FALSE)?;

        dx_DrawCircle(320, 240, 120, green, TRUE, 1)?;

        let font_handle = dx_CreateFontToHandle(ptr::null(), 40, 3, -1, -1, -1, FALSE, -1)?;

        let message = format!("Hello DxLib!: {:?}\0", 1 + 1);
        dx_DrawString(250, 240 - 32, message.as_ptr(), black, 0)?;
        dx_DrawStringToHandle( 0 , 0 , "Wow!!\0".as_ptr(), green , font_handle, 0, 0)?;
        dx_WaitKey()?;

        dx_DxLib_End()?;
    }
    Ok(())
}
