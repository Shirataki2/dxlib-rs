use dxlib_sys::*;
use std::{process::exit, ptr};

const KEY_INPUT_ESCAPE: i32 = 0x01;
const PAD_INPUT_DOWN: i32 = 0x01;
const PAD_INPUT_LEFT: i32 = 0x02;
const PAD_INPUT_RIGHT: i32 = 0x04;
const PAD_INPUT_UP: i32 = 0x08;

const PAD_INPUT_A: i32 = 0x10;
const DX_INPUT_KEY_PAD1: i32 = 0x1001;

fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(640, 480, 32, 60);
        dx_SetMainWindowText("02 Jump\0".as_ptr() as *const i8);

        if dx_DxLib_Init() != 0 {
            exit(1);
        }
        dx_SetDrawScreen(DX_SCREEN_BACK);

        let handle = dx_LoadGraph(
            "./examples/resources/images/Player.bmp".as_ptr() as *const i8,
            FALSE,
        );

        let (mut player_x, mut player_y) = (320, 240);
        let mut jump_power = 0;

        while dx_ProcessMessage() == 0 && dx_CheckHitKey(KEY_INPUT_ESCAPE) == 0 {
            let key = dx_GetJoypadInputState(DX_INPUT_KEY_PAD1);

            player_y += if key & PAD_INPUT_UP > 0 { -3 } else { 0 };
            player_y += if key & PAD_INPUT_DOWN > 0 { 3 } else { 0 };

            player_x += if key & PAD_INPUT_LEFT > 0 { -3 } else { 0 };
            player_x += if key & PAD_INPUT_RIGHT > 0 { 3 } else { 0 };

            player_y -= jump_power;
            jump_power -= 1;
            if player_y > 300 {
                player_y = 300;
                jump_power = 0;
            }

            if (key & PAD_INPUT_A > 0) && player_y == 300 {
                jump_power = 20;
            }

            dx_ClearDrawScreen(ptr::null());

            dx_DrawGraph(player_x, player_y, handle, TRUE);

            dx_ScreenFlip();
        }

        dx_DxLib_End();
    }
    Ok(())
}
