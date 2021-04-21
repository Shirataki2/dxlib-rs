use dxlib_sys::{dx_SetCameraNearFar, dx_SetCameraPositionAndAngle, dx_SetCameraPositionAndTargetAndUpVec, dx_SetCameraPositionAndTarget_UpVecY, dx_SetCameraViewMatrix};
 
use crate::{math::{vector::Vector3, matrix::Matrix4x4, angle::Angle}, error::{I32CodeExt, Result}};

#[derive(Debug, Default)]
pub struct Camera {
    near: f32,
    far: f32,
}

impl Camera {
    pub fn set_near_far(&mut self, near: f32, far: f32) -> Result<()> {
        self.near = near;
        self.far = far;
        unsafe { dx_SetCameraNearFar(near, far).ensure_zero() }
    }

    pub fn set_position_from_look_and_upvec_y(&mut self, position: Vector3<f32>, target: Vector3<f32>) -> Result<()> {
        unsafe { dx_SetCameraPositionAndTarget_UpVecY(position.into(), target.into()).ensure_zero() }
    }

    pub fn set_position_from_look(&mut self, position: Vector3<f32>, target: Vector3<f32>, up: Vector3<f32>) -> Result<()> {
        unsafe { dx_SetCameraPositionAndTargetAndUpVec(position.into(), target.into(), up.into()).ensure_zero() }
    }

    pub fn set_position_from_look_and_angle(&mut self, position: Vector3<f32>, v_rotate: Angle<f32>, h_rotate: Angle<f32>, t_rotate: Angle<f32>) -> Result<()> {
        unsafe { dx_SetCameraPositionAndAngle(position.into(), v_rotate.to_radians(), h_rotate.to_radians(), t_rotate.to_radians()).ensure_zero() }
    }

    pub fn set_position_from_view_matrix(&mut self, matrix: Matrix4x4<f32>) -> Result<()> {
        unsafe { dx_SetCameraViewMatrix(matrix.into()).ensure_zero() }
    }
}
