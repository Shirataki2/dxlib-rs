use crate::{
    color::Color,
    error::{I32CodeExt, Result},
    graphics::image::GraphicModel,
    math::{
        angle::Angle,
        vector::{Vector2, Vector3},
    },
};
use dxlib_sys::{
    consts::*, dx_DrawBillboard3D, dx_DrawCapsule3D, dx_DrawCone3D, dx_DrawLine3D,
    dx_DrawPolygon3D, dx_DrawPolygonIndexed3D, dx_DrawSphere3D, dx_DrawTriangle3D, dx_SetFogColor,
    dx_SetFogEnable, dx_SetFogStartEnd, dx_SetUseZBuffer3D, dx_SetWriteZBuffer3D, Vertex3d,
};
use smart_default::SmartDefault;

use super::Vertex3D;

#[derive(Debug, Clone, Copy, SmartDefault)]
pub struct Line3D {
    pub start: Vector3<f32>,
    #[default(_code = "Vector3::from([1.0, 0.0, 0.0])")]
    pub end: Vector3<f32>,
    pub color: Color<u8>,
}

impl Line3D {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawLine3D(self.start.into(), self.end.into(), self.color.as_u32()).ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy, SmartDefault)]
pub struct Triangle3D {
    pub pos1: Vector3<f32>,
    #[default(_code = "Vector3::from([1.0, 0.0, 1.0])")]
    pub pos2: Vector3<f32>,
    #[default(_code = "Vector3::from([1.0, 1.0, 1.0])")]
    pub pos3: Vector3<f32>,
    pub color: Color<u8>,
    pub filled: bool,
}

impl Triangle3D {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_SetUseZBuffer3D(1).ensure_zero()?;
            dx_SetWriteZBuffer3D(1).ensure_zero()?;
            dx_DrawTriangle3D(
                self.pos1.into(),
                self.pos2.into(),
                self.pos3.into(),
                self.color.as_u32(),
                self.filled as i32,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy, SmartDefault)]
pub struct Sphere3D {
    pub pos: Vector3<f32>,
    #[default = 1.0]
    pub radius: f32,
    #[default = 32]
    pub resolution: u32,
    #[default(_code = "Color::red()")]
    pub diffuse: Color<u8>,
    #[default(_code = "Color::white()")]
    pub specular: Color<u8>,
    pub filled: bool,
}

impl Sphere3D {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_SetUseZBuffer3D(1).ensure_zero()?;
            dx_SetWriteZBuffer3D(1).ensure_zero()?;
            dx_DrawSphere3D(
                self.pos.into(),
                self.radius,
                self.resolution as i32,
                self.diffuse.as_u32(),
                self.specular.as_u32(),
                self.filled as i32,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy, SmartDefault)]
pub struct Capsule3D {
    pub pos1: Vector3<f32>,
    #[default(_code = "Vector3::from([0.0, 3.0, 0.0])")]
    pub pos2: Vector3<f32>,
    #[default = 1.0]
    pub radius: f32,
    #[default = 32]
    pub resolution: u32,
    #[default(_code = "Color::red()")]
    pub diffuse: Color<u8>,
    #[default(_code = "Color::white()")]
    pub specular: Color<u8>,
    pub filled: bool,
}

impl Capsule3D {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_SetUseZBuffer3D(1).ensure_zero()?;
            dx_SetWriteZBuffer3D(1).ensure_zero()?;
            dx_DrawCapsule3D(
                self.pos1.into(),
                self.pos2.into(),
                self.radius,
                self.resolution as i32,
                self.diffuse.as_u32(),
                self.specular.as_u32(),
                self.filled as i32,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, Clone, Copy, SmartDefault)]
pub struct Cone3D {
    #[default(_code = "Vector3::from([0.0, 1.0, 0.0])")]
    pub top: Vector3<f32>,
    pub bottom: Vector3<f32>,
    #[default = 1.0]
    pub radius: f32,
    #[default = 32]
    pub resolution: u32,
    #[default(_code = "Color::red()")]
    pub diffuse: Color<u8>,
    #[default(_code = "Color::white()")]
    pub specular: Color<u8>,
    pub filled: bool,
}

impl Cone3D {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_SetUseZBuffer3D(1).ensure_zero()?;
            dx_SetWriteZBuffer3D(1).ensure_zero()?;
            dx_DrawCone3D(
                self.top.into(),
                self.bottom.into(),
                self.radius,
                self.resolution as i32,
                self.diffuse.as_u32(),
                self.specular.as_u32(),
                self.filled as i32,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug, SmartDefault)]
pub struct Image3D {
    pub image: GraphicModel,
    pub position: Vector3<f32>,
    #[default(_code = "Vector2::from([0.5, 0.5])")]
    pub center: Vector2<f32>,
    #[default = 2.0]
    pub size: f32,
    pub angle: Angle<f32>,
    pub transparent: bool,
}

impl Image3D {
    pub fn draw(&self) -> Result<()> {
        unsafe {
            dx_DrawBillboard3D(
                self.position.into(),
                self.center[0],
                self.center[1],
                self.size,
                *self.angle,
                self.image.handle,
                self.transparent as i32,
                0,
                0,
            )
            .ensure_zero()
        }
    }
}

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Vertex3D>,
    pub image: Option<GraphicModel>,
    pub transparent: bool,
}

impl Polygon {
    pub fn draw(&self) -> Result<()> {
        let vertices = self
            .vertices
            .iter()
            .map(|v| Vertex3d::from(*v))
            .collect::<Vec<_>>();
        let n = self.vertices.len() as i32;
        let image = self
            .image
            .clone()
            .map(|g| g.handle)
            .unwrap_or(DX_NONE_GRAPH);
        unsafe {
            dx_DrawPolygon3D(vertices.as_ptr(), n, image, self.transparent as i32).ensure_zero()
        }
    }
}

pub struct IndexedPolygon {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<(usize, usize, usize)>,
    pub image: Option<GraphicModel>,
    pub transparent: bool,
}

impl IndexedPolygon {
    pub fn draw(&self) -> Result<()> {
        let vertices = self
            .vertices
            .iter()
            .map(|v| Vertex3d::from(*v))
            .collect::<Vec<_>>();
        let n_vertex = self.vertices.len() as i32;
        let mut indices = Vec::new();
        for i in 0..self.indices.len() {
            indices.push(self.indices[i].0 as u16);
            indices.push(self.indices[i].1 as u16);
            indices.push(self.indices[i].2 as u16);
        }
        let n_index = indices.len() as i32;
        let image = self
            .image
            .clone()
            .map(|g| g.handle)
            .unwrap_or(DX_NONE_GRAPH);
        unsafe {
            dx_DrawPolygonIndexed3D(
                vertices.as_ptr(),
                n_vertex,
                indices.as_ptr(),
                n_index,
                image,
                self.transparent as i32,
            )
            .ensure_zero()
        }
    }
}

pub struct Fog;

impl Fog {
    pub fn enable(x: bool) -> Result<()> {
        unsafe { dx_SetFogEnable(x as i32).ensure_zero() }
    }

    pub fn set_color(c: Color<u8>) -> Result<()> {
        unsafe { dx_SetFogColor(c.r as i32, c.g as i32, c.b as i32).ensure_zero() }
    }

    pub fn set_fog_start_end(start: f32, end: f32) -> Result<()> {
        unsafe { dx_SetFogStartEnd(start, end).ensure_zero() }
    }
}
