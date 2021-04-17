use dxlib_sys::*;
use std::{process::exit, ptr};

fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(640, 480, 32, 60);
        dx_SetMainWindowText("01 Key Input\0".as_ptr() as *const i8);

        if dx_DxLib_Init() != 0 {
            exit(1);
        }
        dx_SetDrawScreen(DX_SCREEN_BACK);

        let handle = dx_LoadGraph(
            "./examples/resources/images/Player.bmp".as_ptr() as *const i8,
            FALSE,
        );

        let (mut player_x, mut player_y) = (320, 240);

        while dx_ProcessMessage() == 0 && dx_CheckHitKey(KEY_INPUT_ESCAPE) == 0 {
            let key = dx_GetJoypadInputState(DX_INPUT_KEY_PAD1);

            player_y += if key & PAD_INPUT_UP > 0 { -3 } else { 0 };
            player_y += if key & PAD_INPUT_DOWN > 0 { 3 } else { 0 };

            player_x += if key & PAD_INPUT_LEFT > 0 { -3 } else { 0 };
            player_x += if key & PAD_INPUT_RIGHT > 0 { 3 } else { 0 };

            dx_ClearDrawScreen(ptr::null());

            dx_DrawGraph(player_x, player_y, handle, TRUE);

            dx_ScreenFlip();
        }

        dx_DxLib_End();
    }
    Ok(())
}
