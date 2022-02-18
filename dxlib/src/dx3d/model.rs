use num_traits::{One, Zero};
use anyhow::Context as _;
use dxlib_sys::{
    consts::*, dx_MV1DeleteModel, dx_MV1DrawFrame, dx_MV1DrawMesh, dx_MV1DrawModel,
    dx_MV1DrawTriangleList, dx_MV1LoadModel, dx_MV1SetLoadModelUsePhysicsMode, dx_MV1SetPosition, dx_MV1SetScale, dx_MV1SetRotationXYZ, dx_MV1SetMatrix, dx_MV1GetPosition, dx_MV1GetScale, dx_MV1GetRotationXYZ, dx_MV1GetMatrix, dx_MV1SetVisible, dx_MV1GetVisible,
};
use std::path::Path;

use crate::{
    error::{I32CodeExt, Result},
    math::vector::Vector3,
    utils::to_sjis_bytes, prelude::*,
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
    scale: Vector3<f32>,
    rotation: Vector3<Angle<f32>>,
}

impl Mv1Model {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Mv1Model> {
        let path = to_sjis_bytes(&path.as_ref().to_string_lossy());
        let path = path.as_ptr() as *const i8;
        let handle = unsafe {
            dx_MV1LoadModel(path)
                .ensure_not_minus1()
                .context("Model Load Failed")?
        };
        let pos = Vector3::default();
        let scale = Vector3::one();
        let rotation = Vector3::zero();
        unsafe {
            dx_MV1SetLoadModelUsePhysicsMode(PhysicsMode::Disable as i32).ensure_not_minus1()?;
        }
        Ok(Mv1Model { handle, pos, scale, rotation })
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

    pub fn set_scale(&mut self, scale: Vector3<f32>) -> Result<()> {
        self.scale = scale;
        unsafe { dx_MV1SetScale(self.handle, scale.into()).ensure_zero() }
    }

    pub fn get_scale(&self) -> Vector3<f32> {
        self.scale
    }

    pub fn set_rotation(&mut self, rotation: Vector3<Angle<f32>>) -> Result<()> {
        self.rotation = rotation;
        unsafe { dx_MV1SetRotationXYZ(self.handle, rotation.into()).ensure_zero() }
    }

    pub fn get_rotation(&self) -> Vector3<Angle<f32>> {
        self.rotation
    }

    pub fn set_matrix(&mut self, matrix: Matrix4x4<f32>) -> Result<()> {
        unsafe {
            dx_MV1SetMatrix(self.handle, matrix.into()).ensure_zero()?;
            let pos: Vector3<f32> = dx_MV1GetPosition(self.handle).into();
            self.pos = pos;
            let scale: Vector3<f32> = dx_MV1GetScale(self.handle).into();
            self.scale = scale;
            let rotation: Vector3<f32> = dx_MV1GetRotationXYZ(self.handle).into();
            self.rotation = Vector3::from([
                Angle::from_radians(rotation[0]),
                Angle::from_radians(rotation[1]),
                Angle::from_radians(rotation[2]),
            ]);
        }

        Ok(())
    }

    pub fn get_matrix(&self) -> Result<Matrix4x4<f32>> {
        unsafe {
            let mat = dx_MV1GetMatrix(self.handle);
            Ok(mat.into())
        }
    }

    pub fn set_visible(&mut self, flag: bool) -> Result<()> {
        unsafe { dx_MV1SetVisible(self.handle, flag as i32).ensure_zero() }
    }

    pub fn get_visible(&self) -> Result<bool> {
        unsafe {
            let flag = dx_MV1GetVisible(self.handle).ensure_not_minus1()?;
            Ok(flag != 0)
        }
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
