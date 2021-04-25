use dxlib_sys::*;
use encoding_rs::SHIFT_JIS;
use std::{process::exit, ptr};

const MESSAGE: &str = include_str!("06-soundnovel.txt");
const FONT_SIZE: i32 = 24;

fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(640, 480, 32, 60);
        dx_SetMainWindowText("06 Sound Novel Like\0".as_ptr() as *const i8);

        if dx_DxLib_Init() != 0 {
            exit(1);
        }

        let mut pos_x = 0;
        let mut pos_y = 0;
        let mut ln = 0;
        let mut ch = 0;

        dx_SetFontSize(FONT_SIZE);

        while dx_ProcessMessage() == 0 && dx_CheckHitKey(KEY_INPUT_ESCAPE) == 0 {
            let c = MESSAGE.split('\n').collect::<Vec<_>>()[ln]
                .chars()
                .collect::<Vec<_>>()[ch];
            match c {
                '@' => {
                    pos_y += 1;
                    pos_x = 0;
                    if pos_y * FONT_SIZE + FONT_SIZE > 480 {
                        let temp_graph = dx_MakeGraph(640, 480, TRUE);
                        dx_GetDrawScreenGraph(0, 0, 640, 480, temp_graph, FALSE);
                        dx_DrawGraph(0, -FONT_SIZE, temp_graph, FALSE);
                        dx_DrawBox(0, 480 - FONT_SIZE, 640, 480, 0, TRUE);
                        pos_y -= 1;
                        dx_DeleteGraph(temp_graph, FALSE);
                    }
                    ch += 1;
                }
                'B' => {
                    dx_WaitKey();
                    ch += 1;
                }
                'E' => {
                    break;
                }
                'C' => {
                    dx_ClearDrawScreen(ptr::null());
                    pos_x = 0;
                    pos_y = 0;
                    ch += 1;
                }
                c => {
                    let s = format!("{}\0", c);
                    let (bytes, _, _) = SHIFT_JIS.encode(&s);
                    let bytes = bytes.as_ptr() as *const i8;
                    dx_DrawString(
                        pos_x * FONT_SIZE,
                        pos_y * FONT_SIZE,
                        bytes,
                        dx_GetColor(255, 255, 255),
                        0,
                    );
                    ch += 1;
                    pos_x += 1;
                    dx_WaitTimer(10);
                    if pos_x * FONT_SIZE + FONT_SIZE > 640 {
                        pos_y += 1;
                        pos_x = 0;
                        if pos_y * FONT_SIZE + FONT_SIZE > 480 {
                            let temp_graph = dx_MakeGraph(640, 480, TRUE);
                            dx_GetDrawScreenGraph(0, 0, 640, 480, temp_graph, FALSE);
                            dx_DrawGraph(0, -FONT_SIZE, temp_graph, FALSE);
                            dx_DrawBox(0, 480 - FONT_SIZE, 640, 480, 0, TRUE);
                            pos_y -= 1;
                            dx_DeleteGraph(temp_graph, FALSE);
                        }
                    }
                }
            }
            if MESSAGE.split('\n').collect::<Vec<_>>()[ln].chars().count() == ch {
                ln += 1;
                ch = 0;
            }
        }

        dx_DxLib_End();
    }
    Ok(())
}
