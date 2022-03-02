use std::{ffi::CString, result::Result as StdResult, time};

use crate::{
    error::{DxLibError, I32CodeExt, Result},
    math::vector::Vector4,
    plugin::Plugin,
    screen::Screen,
    utils::to_sjis_bytes,
};
use dxlib_sys::*;
use smart_default::SmartDefault;
use winapi::shared::windef::HWND;

const DEFAULT_WIDTH: usize = 1280;
const DEFAULT_HEIGHT: usize = 720;

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Direct3D {
    Dx9 = DX_DIRECT3D_9,
    Dx9Ex = DX_DIRECT3D_9EX,
    Dx11 = DX_DIRECT3D_11,
    None = DX_DIRECT3D_NONE,
}

#[derive(Debug, Clone, Copy)]
pub enum ColorBitDepth {
    Sixteen = 16,
    ThirtyTwo = 32,
}

#[derive(Debug, Clone, Copy)]
pub enum ScreenMode {
    Fullscreen = 0,
    Windowed = 1,
}

impl Default for ColorBitDepth {
    fn default() -> ColorBitDepth {
        ColorBitDepth::ThirtyTwo
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowStyle {
    Default = 0,
    NoTitleBar = 1,
    NoTitleBarNoBorder = 2,
    NoBorder = 3,
    None = 4,
    NoMinimize = 5,
    WithToolBar = 6,
    WithMaximizeInitalizeNormal = 7,
    WithMaximizeInitializeMaximum = 8,
    NoSolidFrame = 9,
    WithMaximizeNoSolidFrame = 10,
    NoCloseNoMinimize = 11,
}

impl Default for WindowStyle {
    fn default() -> WindowStyle {
        WindowStyle::Default
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Application {
    width: usize,
    height: usize,
    timer: time::Instant,
    frame: u128,
    color_depth: ColorBitDepth,
    refresh_rate: i32,
    screen_mode: ScreenMode,
    window_handle: Option<HWND>,
    window_style: WindowStyle,
    d3d: Direct3D,
    pub screen: Screen,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub fn process_message(&mut self) -> Result<()> {
        let code = unsafe { dx_ProcessMessage() };
        if code != 0 {
            return Err(DxLibError::MessageProcessingFailed);
        }
        self.frame += 1;
        Ok(())
    }

    pub fn get_window_handle(&mut self) -> Result<HWND> {
        match self.window_handle {
            None => {
                let hwnd = unsafe { dx_GetMainWindowHandle() };
                self.window_handle = Some(hwnd);
                Ok(hwnd)
            }
            Some(hwnd) => Ok(hwnd),
        }
    }

    pub fn run<F: FnOnce(&Application) -> Result<()>>(&self, f: F) -> Result<()> {
        f(self)?;
        Ok(())
    }

    pub fn screenshot(&self, rect: Option<Vector4<i32>>, name: &str) -> Result<()> {
        let rect = rect.map_or(
            Vector4::from([0, 0, self.width as i32, self.height as i32]),
            |f| f,
        );
        let name = CString::new(name)?;
        unsafe {
            dx_SaveDrawScreen(
                rect[0],
                rect[1],
                rect[2],
                rect[3],
                name.as_ptr(),
                DX_IMAGESAVETYPE_BMP,
                80,
                1,
                -1,
            )
            .ensure_zero()
        }
    }

    fn close(&self) -> Result<i32> {
        let code = unsafe { dx_DxLib_End() };
        Ok(code)
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

#[derive(SmartDefault)]
pub struct ApplicationBuilder {
    width: Option<usize>,
    height: Option<usize>,
    color_depth: Option<ColorBitDepth>,
    refresh_rate: Option<i32>,
    screen_mode: Option<ScreenMode>,
    title: Option<String>,
    d3d: Option<Direct3D>,
    window_style: Option<WindowStyle>,
    transparent: bool,
    #[default = true]
    vsync: bool,
}

impl ApplicationBuilder {
    pub fn screen_size(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn color_depth(&mut self, depth: ColorBitDepth) -> &mut Self {
        self.color_depth = Some(depth);
        self
    }

    pub fn refresh_rate(&mut self, refresh_rate: i32) -> &mut Self {
        self.refresh_rate = Some(refresh_rate);
        self
    }

    pub fn screen_mode(&mut self, screen_mode: ScreenMode) -> &mut Self {
        self.screen_mode = Some(screen_mode);
        self
    }

    pub fn title(&mut self, title: &str) -> &mut Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn direct3d(&mut self, direct3d: Direct3D) -> &mut Self {
        self.d3d = Some(direct3d);
        self
    }

    pub fn transparent_window(&mut self, flag: bool) -> &mut Self {
        self.transparent = flag;
        self
    }

    pub fn vsync(&mut self, flag: bool) -> &mut Self {
        self.vsync = flag;
        self
    }

    pub fn window_style(&mut self, style: WindowStyle) -> &mut Self {
        self.window_style = Some(style);
        self
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> StdResult<&mut Self, P::Error> {
        plugin.build(self)?;
        Ok(self)
    }

    pub fn build(&mut self) -> Result<Application> {
        let width = self.width.unwrap_or(DEFAULT_WIDTH);
        let height = self.height.unwrap_or(DEFAULT_HEIGHT);
        let color_depth = self.color_depth.unwrap_or_default();
        let refresh_rate = self.refresh_rate.unwrap_or(60);
        let screen_mode = self.screen_mode.unwrap_or(ScreenMode::Windowed);
        let window_style = self.window_style.unwrap_or_default();
        let d3d = self.d3d.unwrap_or(Direct3D::Dx9Ex);

        let code = unsafe { dx_SetUseDirect3DVersion(d3d as i32) };
        if code != 0 {
            return Err(DxLibError::NonZeroReturned);
        }

        let code = unsafe { dx_SetWindowStyleMode(window_style as i32) };
        if code != 0 {
            return Err(DxLibError::NonZeroReturned);
        }

        if self.transparent {
            unsafe {
                dx_SetUseBackBufferTransColorFlag(TRUE).ensure_zero()?;
                dx_SetUsePremulAlphaConvertLoad(TRUE).ensure_zero()?;
            }
        }

        unsafe {
            dx_SetWaitVSyncFlag(self.vsync as i32).ensure_zero()?;
        }

        let _: Option<()> = self.title.clone().and_then(|title| {
            let title = to_sjis_bytes(&title);
            let title = title.as_ptr() as *const i8;
            unsafe {
                dx_SetWindowText(title);
            }
            None
        });

        let code = unsafe {
            dx_SetGraphMode(
                width as i32,
                height as i32,
                color_depth as i32,
                refresh_rate,
            )
        };
        if code != 0 {
            return Err(DxLibError::NonZeroReturned);
        }

        let code = unsafe { dx_ChangeWindowMode(screen_mode as i32) };
        if code != 0 {
            return Err(DxLibError::NonZeroReturned);
        }

        let code = unsafe { dx_DxLib_Init() };
        if code != 0 {
            return Err(DxLibError::InitializeFailed);
        }

        let screen = Screen::default();

        Ok(Application {
            width,
            height,
            color_depth,
            refresh_rate,
            screen_mode,
            screen,
            window_style,
            window_handle: None,
            timer: time::Instant::now(),
            frame: 0,
            d3d,
        })
    }
}
