use dxlib_sys::*;
use std::{process::exit, ptr};

fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(380, 720, 32, 60);

        dx_Live2D_SetCubism4CoreDLLPath("Live2DCubismCore.dll\0".as_ptr() as *const i8);

        if dx_DxLib_Init() != 0 {
            exit(1);
        }

        let model = dx_Live2D_LoadModel(
            "./examples/resources/Hiyori/Hiyori.model3.json\0".as_ptr() as *const i8,
        );

        dx_Live2D_Model_SetExtendRate(model, 4.0, 4.0);
        dx_Live2D_Model_SetTranslate(model, 0.0, 0.0);

        dx_SetDrawScreen(DX_SCREEN_BACK);

        while dx_ProcessMessage() == 0 {
            dx_ClearDrawScreen(ptr::null());

            if dx_Live2D_Model_IsMotionFinished(model) == 1 {
                dx_Live2D_Model_StartMotion(model, "Idle\0".as_ptr() as *const i8, dx_GetRand(8));
            }

            dx_Live2D_Model_Update(model, 1.0 / 60.0);

            dx_Live2D_RenderBegin();

            dx_Live2D_Model_Draw(model);

            dx_Live2D_RenderEnd();

            dx_ScreenFlip();
        }
        dx_Live2D_DeleteModel(model);
        dx_DxLib_End();
    }
    Ok(())
}
