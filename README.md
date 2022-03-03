# Rust DxLib

DXライブラリのRust FFIです．

`dxlib-sys`がグルーコードです．全API関数に対して愚直に呼び出すだけの低レベルなAPIを提供します．

`dxlib`は上記のcrateをもとにRustで扱いやすいように定義しなおしたものです．

公開API関数の網羅状況は[こちら](PROGRESS.md)をご参照ください．

どちらを利用してもアプリケーションを起動させることは可能です．

## 準備

[こちら](./SETUP.md)を参考にしてください．

## `dxlib-sys`を利用する場合

Live2Dモデルをロードし，ランダムにアニメーションするスクリプトの例です．

```rust
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
            "./resources/Hiyori/Hiyori.model3.json\0".as_ptr() as *const i8,
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
```

## `dxlib`を利用する場合(作成中)

```rust
use std::time::Duration;

use dxlib::{
    application::Application,
    live2d::{Live2DModel, Live2DRenderer},
    math::Vector2,
    screen::DrawScreen,
};
use rand::{thread_rng, Rng};

fn main() -> anyhow::Result<()> {
    let mut app = Application::builder().screen_size(380, 740).build()?;

    let mut rng = thread_rng();

    let mut renderer = Live2DRenderer::new("Live2DCubismCore.dll")?;
    let model = Live2DModel::load("./resources/Hiyori/Hiyori.model3.json")?;
    renderer.add_model(&model)?;
    model.scale(Vector2::from(&[4.0, 4.0]))?;

    app.screen.set_draw_screen(DrawScreen::Back)?;

    while app.process_message().is_ok() {
        app.screen.clear()?;
        if model.is_motion_finished()? {
            model.start_motion("Idle", rng.gen_range(0..8))?;
        }
        model.update(Duration::from_millis(25))?;
        renderer.render()?;
        app.screen.flip()?;
    }

    Ok(())
}
```