use dxlib_sys::{
    consts::*, dx_MV1DeleteModel, dx_MV1DrawFrame, dx_MV1DrawMesh, dx_MV1DrawModel,
    dx_MV1DrawTriangleList, dx_MV1LoadModel, dx_MV1SetLoadModelUsePhysicsMode, dx_MV1SetPosition,
};
use std::path::Path;

use crate::{
    error::{I32CodeExt, Result},
    math::vector::Vector3,
    utils::to_sjis_bytes,
};

#[repr(i32)]
pub enum PhysicsMode {
    Disable = DX_LOADMODEL_PHYSICS_DISABLE,
    Realtime = DX_LOADMODEL_PHYSICS_REALTIME,
    LoadCalc = DX_LOADMODEL_PHYSICS_LOADCALC,
}

#[derive(Debug)]
pub struct Mv1Model {
    pub handle: i32,
    pos: Vector3<f32>,
}

impl Mv1Model {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Mv1Model> {
        let path = to_sjis_bytes(&path.as_ref().to_string_lossy());
        let path = path.as_ptr() as *const i8;
        let handle = unsafe { dx_MV1LoadModel(path).ensure_not_minus1()? };
        let pos = Vector3::default();
        unsafe {
            dx_MV1SetLoadModelUsePhysicsMode(PhysicsMode::Disable as i32).ensure_not_minus1()?;
        }
        Ok(Mv1Model { handle, pos })
    }

    pub fn load_with_physics<P: AsRef<Path>>(path: P, mode: PhysicsMode) -> Result<Mv1Model> {
        unsafe {
            dx_MV1SetLoadModelUsePhysicsMode(mode as i32).ensure_not_minus1()?;
        }
        Self::load(path)
    }

    pub fn set_position(&mut self, pos: Vector3<f32>) -> Result<()> {
        self.pos = pos;
        unsafe { dx_MV1SetPosition(self.handle, pos.into()).ensure_zero() }
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.pos
    }

    pub fn draw(&mut self) -> Result<()> {
        unsafe { dx_MV1DrawModel(self.handle).ensure_zero() }
    }

    pub fn draw_frame(&mut self, frame: i32) -> Result<()> {
        unsafe { dx_MV1DrawFrame(self.handle, frame).ensure_zero() }
    }

    pub fn draw_mesh(&mut self, mesh: i32) -> Result<()> {
        unsafe { dx_MV1DrawMesh(self.handle, mesh).ensure_zero() }
    }

    pub fn draw_triangle(&mut self, triangle: i32) -> Result<()> {
        unsafe { dx_MV1DrawTriangleList(self.handle, triangle).ensure_zero() }
    }

    fn close(&mut self) -> Result<()> {
        unsafe { dx_MV1DeleteModel(self.handle).ensure_zero() }
    }
}

impl Drop for Mv1Model {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
