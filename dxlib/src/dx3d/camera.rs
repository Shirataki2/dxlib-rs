use dxlib_sys::{
    dx_CheckCameraViewClip, dx_CheckCameraViewClip_Box, dx_ConvScreenPosToWorldPos,
    dx_ConvWorldPosToScreenPos, dx_GetCameraProjectionMatrix, dx_GetCameraViewMatrix,
    dx_SetCameraDotAspect, dx_SetCameraNearFar, dx_SetCameraPositionAndAngle,
    dx_SetCameraPositionAndTargetAndUpVec, dx_SetCameraPositionAndTarget_UpVecY,
    dx_SetCameraScreenCenter, dx_SetCameraViewMatrix, dx_SetupCamera_Ortho,
    dx_SetupCamera_Perspective,
};
use smart_default::SmartDefault;

use crate::{
    error::{DxLibError, I32CodeExt, Result},
    math::{
        angle::Angle,
        matrix::Matrix4x4,
        vector::{Vector2, Vector3},
    },
};

#[derive(Debug, Clone, Copy)]
pub enum ProjectionType {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
}

#[derive(Debug, Clone, Copy)]

pub struct PerspectiveProjection(f32);

impl PerspectiveProjection {
    pub fn new(fov: f32) -> PerspectiveProjection {
        PerspectiveProjection(fov)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrthographicProjection(f32);

impl OrthographicProjection {
    pub fn new(size: f32) -> OrthographicProjection {
        OrthographicProjection(size)
    }
}

#[derive(Debug, SmartDefault)]
pub struct Camera {
    near: f32,
    far: f32,
    #[default(
        _code = "ProjectionType::Perspective(PerspectiveProjection::new(std::f32::consts::FRAC_PI_3))"
    )]
    projection_type: ProjectionType,
}

impl Camera {
    pub fn set_projection_type(&mut self, projection_type: ProjectionType) -> Result<()> {
        self.projection_type = projection_type;
        use ProjectionType::*;
        match projection_type {
            Perspective(prj) => unsafe {
                dx_SetupCamera_Perspective(prj.0).ensure_zero()?;
            },
            Orthographic(prj) => unsafe {
                dx_SetupCamera_Ortho(prj.0).ensure_zero()?;
            },
        };
        Ok(())
    }

    pub fn set_dot_aspect(&mut self, aspect: f32) -> Result<()> {
        unsafe { dx_SetCameraDotAspect(aspect).ensure_zero() }
    }

    pub fn set_near_far(&mut self, near: f32, far: f32) -> Result<()> {
        self.near = near;
        self.far = far;
        unsafe { dx_SetCameraNearFar(near, far).ensure_zero() }
    }

    pub fn set_position_from_look_and_upvec_y(
        &mut self,
        position: Vector3<f32>,
        target: Vector3<f32>,
    ) -> Result<()> {
        unsafe {
            dx_SetCameraPositionAndTarget_UpVecY(position.into(), target.into()).ensure_zero()
        }
    }

    pub fn set_position_from_look(
        &mut self,
        position: Vector3<f32>,
        target: Vector3<f32>,
        up: Vector3<f32>,
    ) -> Result<()> {
        unsafe {
            dx_SetCameraPositionAndTargetAndUpVec(position.into(), target.into(), up.into())
                .ensure_zero()
        }
    }

    pub fn set_position_from_look_and_angle(
        &mut self,
        position: Vector3<f32>,
        v_rotate: Angle<f32>,
        h_rotate: Angle<f32>,
        t_rotate: Angle<f32>,
    ) -> Result<()> {
        unsafe {
            dx_SetCameraPositionAndAngle(
                position.into(),
                v_rotate.to_radians(),
                h_rotate.to_radians(),
                t_rotate.to_radians(),
            )
            .ensure_zero()
        }
    }

    pub fn set_position_from_view_matrix(&mut self, matrix: Matrix4x4<f32>) -> Result<()> {
        unsafe { dx_SetCameraViewMatrix(matrix.into()).ensure_zero() }
    }

    pub fn get_screen_position(
        &self,
        world_position: Vector3<f32>,
    ) -> Result<Option<Vector2<f32>>> {
        let pos = unsafe { dx_ConvWorldPosToScreenPos(world_position.into()) };
        match pos.z {
            z if z <= 0.0 || 1.0 <= z => Ok(None),
            _ => Ok(Some(Vector2::from([pos.x, pos.y]))),
        }
    }
    pub fn set_vanishing_point(&mut self, point: Vector2<f32>) -> Result<()> {
        unsafe { dx_SetCameraScreenCenter(point[0], point[1]).ensure_zero() }
    }

    pub fn get_world_position(
        &self,
        screen_position: Vector2<f32>,
        depth: f32,
    ) -> Result<Vector3<f32>> {
        if 0.0 > depth || depth < 1.0 {
            return Err(DxLibError::InvalidRange(String::from("Depth"), 0, 1));
        }
        let v = screen_position;
        let screen_pos = Vector3::from([v[0], v[1], depth]);
        let pos = unsafe { dx_ConvScreenPosToWorldPos(screen_pos.into()) };
        Ok(Vector3::from([pos.x, pos.y, pos.z]))
    }

    pub fn get_view_matrix(&self) -> Result<Matrix4x4<f32>> {
        let mat = unsafe { dx_GetCameraViewMatrix() };
        Ok(Matrix4x4::from(mat.m))
    }

    pub fn get_projection_matrix(&self) -> Result<Matrix4x4<f32>> {
        let mat = unsafe { dx_GetCameraProjectionMatrix() };
        Ok(Matrix4x4::from(mat.m))
    }

    pub fn is_point_inside_clip_view(&mut self, point: Vector3<f32>) -> Result<bool> {
        Ok(unsafe { dx_CheckCameraViewClip(point.into()).ensure_positive()? == 1 })
    }

    pub fn is_box_inside_clip_view(&mut self, v1: Vector3<f32>, v2: Vector3<f32>) -> Result<bool> {
        Ok(unsafe { dx_CheckCameraViewClip_Box(v1.into(), v2.into()).ensure_positive()? == 1 })
    }
}
