# Rust DxLib

DXライブラリのDLLを叩いてRust上で動かします．

`dxlib-sys`がDLLとのグルーコードです．全API関数に対して愚直な型で呼び出すだけの低レベルなAPIを提供します．

`dxlib`は上記のcrateをもとにRustで扱いやすいように型を定義しなおしたものです．

どちらを利用してもアプリケーションを起動させることは可能です．

## `dxlib-sys`を利用する場合

```rust
use dxlib_sys::*;
use std::{process::exit, ptr};

// "テスト\0"のshift-jis形式
const INIT_TITLE: &'static [u8] = &[ 0x83, 0x65, 0x83, 0x58, 0x83, 0x67, 0x00 ];

fn main() -> anyhow::Result<()> {
    unsafe {
        dx_ChangeWindowMode(TRUE)?;        
        dx_SetGraphMode(640, 480, 32, 60)?;
        dx_SetMainWindowText(INIT_TITLE.as_ptr())?;

        if dx_DxLib_Init()? != 0 {
            exit(1);
        }
        
        let handle = dx_LoadGraph("lena.png".as_ptr(), FALSE)?;

        while dx_ProcessMessage()? == 0 {
            dx_ClearDrawScreen(ptr::null())?;
            dx_SetDrawScreen(DX_SCREEN_BACK)?;

            dx_DrawGraph(0, 0, handle, 0)?;

            dx_ScreenFlip()?;
        }

        dx_DxLib_End()?;
    }
    Ok(())
}
```

## `dxlib`を利用する場合(作成中)

展望としては以下のコードのunsafeの個所をすべてラッピングすることです．

```rust
use dxlib_sys::*;
use dxlib::application::Application;

fn main() -> anyhow::Result<()>{
    Application::builder()
        .screen_size(600, 600)
        .build()?
        .run(|app| {
            unsafe {
                dx_LoadGraphScreen(0, 0, "lena.png\0".as_ptr(), 0).unwrap();

                while dx_CheckHitKeyAll(7).unwrap() == 0 {
                    app.process_message().unwrap();
                }
            };
            Ok(())
        })?;
}
```