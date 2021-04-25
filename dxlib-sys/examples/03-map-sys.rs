#![allow(clippy::needless_range_loop)]

use dxlib_sys::*;
use std::process::exit;
const MAP_SIZE: usize = 64;
const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 8;

const MAP: [[i32; MAP_WIDTH]; MAP_HEIGHT] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 1, 0, 1, 1, 1, 1, 1, 1, 0],
    [0, 1, 0, 1, 1, 0, 0, 0, 1, 0],
    [0, 1, 1, 1, 1, 0, 0, 0, 1, 0],
    [0, 1, 0, 1, 0, 0, 0, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(640, 512, 32, 60);
        dx_SetMainWindowText("03 Map\0".as_ptr() as *const i8);

        if dx_DxLib_Init() != 0 {
            exit(1);
        }

        for i in 0..MAP_HEIGHT {
            for j in 0..MAP_WIDTH {
                if MAP[i][j] == 0 {
                    dx_DrawBox(
                        (j * MAP_SIZE) as i32,
                        (i * MAP_SIZE) as i32,
                        ((j + 1) * MAP_SIZE) as i32,
                        ((i + 1) * MAP_SIZE) as i32,
                        dx_GetColor(255, 0, 0),
                        TRUE,
                    );
                }
            }
        }

        dx_WaitKey();

        dx_DxLib_End();
    }
    Ok(())
}
