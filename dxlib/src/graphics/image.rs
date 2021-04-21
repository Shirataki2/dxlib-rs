use std::path::Path;

use dxlib_sys::{
    consts::*, dx_DeleteGraph, dx_DrawRotaGraph3, dx_LoadGraph, dx_MakeGraph, dx_SetDrawBlendMode,
};
use smart_default::SmartDefault;

use crate::{
    error::{I32CodeExt, Result},
    math::{angle::Angle, vector::Vector2},
    utils::to_sjis_bytes,
};

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum BlendMode {
    NoBlend = DX_BLENDMODE_NOBLEND,
    Alpha = DX_BLENDMODE_ALPHA,
    Add = DX_BLENDMODE_ADD,
    Sub = DX_BLENDMODE_SUB,
    Mul = DX_BLENDMODE_MUL,
    Inversed = DX_BLENDMODE_INVSRC,
    PmaAlpha = DX_BLENDMODE_PMA_ALPHA,
    PmaAdd = DX_BLENDMODE_PMA_ADD,
    PmaSub = DX_BLENDMODE_PMA_SUB,
    PmaInversed = DX_BLENDMODE_PMA_INVSRC,
}

#[derive(Debug, SmartDefault, Clone)]
pub struct GraphicModel {
    pub(crate) handle: i32,
    pub position: Vector2<i32>,
    pub pivot: Vector2<i32>,
    #[default(_code = "Vector2::<f64>::from([1.0, 1.0])")]
    pub scale: Vector2<f64>,
    pub rotation: Angle<f64>,
    #[default = true]
    pub transparent: bool,
    pub xturned: bool,
    pub yturned: bool,
    #[default(_code = "(BlendMode::NoBlend, 255)")]
    blend: (BlendMode, u8),
}

impl GraphicModel {
    pub fn new(width: i32, height: i32) -> Result<Self> {
        let handle = unsafe { dx_MakeGraph(width, height, 0).ensure_not_minus1()? };
        Ok(Self {
            handle,
            ..Default::default()
        })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = to_sjis_bytes(&path.as_ref().to_string_lossy());
        let path = path.as_ptr() as *const i8;
        let handle = unsafe { dx_LoadGraph(path, 0).ensure_not_minus1()? };
        Ok(Self {
            handle,
            ..Default::default()
        })
    }

    pub fn set_position(&mut self, pos: Vector2<i32>) -> Result<()> {
        self.position = pos;
        Ok(())
    }

    pub fn set_scale(&mut self, scale: Vector2<f64>) -> Result<()> {
        self.scale = scale;
        Ok(())
    }

    pub fn set_pivot(&mut self, pivot: Vector2<i32>) -> Result<()> {
        self.pivot = pivot;
        Ok(())
    }

    pub fn set_rotation(&mut self, angle: Angle<f64>) -> Result<()> {
        self.rotation = angle;
        Ok(())
    }

    pub fn set_transparent(&mut self, transparent: bool) -> Result<()> {
        self.transparent = transparent;
        Ok(())
    }

    pub fn set_turned(&mut self, horizontal: bool, vertical: bool) -> Result<()> {
        self.xturned = horizontal;
        self.yturned = vertical;
        Ok(())
    }

    pub fn set_blendmode(&mut self, blend: BlendMode, param: u8) -> Result<()> {
        self.blend = (blend, param);
        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        unsafe {
            let (mode, param) = self.blend;
            dx_SetDrawBlendMode(mode as i32, param as i32).ensure_zero()?;
            dx_DrawRotaGraph3(
                self.position[0],
                self.position[1],
                self.pivot[0],
                self.pivot[1],
                self.scale[0],
                self.scale[1],
                self.rotation.normalized(),
                self.handle,
                self.transparent as i32,
                self.xturned as i32,
                self.yturned as i32,
            )
            .ensure_zero()?;
            dx_SetDrawBlendMode(BlendMode::NoBlend as i32, 0).ensure_zero()
        }
    }

    fn close(&self) -> Result<()> {
        unsafe { dx_DeleteGraph(self.handle, 0).ensure_zero() }
    }
}

impl Drop for GraphicModel {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
