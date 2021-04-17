use crate::{
    error::{DxLibError, Result},
    screen::Screen,
};
use dxlib_sys::*;

const DEFAULT_WIDTH: usize = 1280;
const DEFAULT_HEIGHT: usize = 720;

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

#[derive(Debug, Clone)]
pub struct Application {
    width: usize,
    height: usize,
    color_depth: ColorBitDepth,
    refresh_rate: i32,
    screen_mode: ScreenMode,
    pub screen: Screen,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        ApplicationBuilder::default()
    }

    pub fn process_message(&self) -> Result<()> {
        let code = unsafe { dx_ProcessMessage() };
        if code != 0 {
            return Err(DxLibError::MessageProcessingFailed);
        }
        return Ok(());
    }

    pub fn run<F: FnOnce(&Application) -> Result<()>>(&self, f: F) -> Result<()> {
        f(self)?;
        Ok(())
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

#[derive(Default)]
pub struct ApplicationBuilder {
    width: Option<usize>,
    height: Option<usize>,
    color_depth: Option<ColorBitDepth>,
    refresh_rate: Option<i32>,
    screen_mode: Option<ScreenMode>,
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

    pub fn build(&mut self) -> Result<Application> {
        let width = self.width.unwrap_or(DEFAULT_WIDTH);
        let height = self.height.unwrap_or(DEFAULT_HEIGHT);
        let color_depth = self.color_depth.unwrap_or_default();
        let refresh_rate = self.refresh_rate.unwrap_or(60);
        let screen_mode = self.screen_mode.unwrap_or(ScreenMode::Windowed);

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
        })
    }
}

#[test]
fn test_simple_app() {
    Application::builder()
        .screen_size(600, 600)
        .build()
        .unwrap()
        .run(|app| {
            unsafe {
                dx_LoadGraphScreen(0, 0, "lena.png\0".as_ptr() as *const i8, 0);

                while dx_CheckHitKeyAll(7) == 0 {
                    app.process_message().unwrap();
                }
            };
            Ok(())
        })
        .unwrap();
}
