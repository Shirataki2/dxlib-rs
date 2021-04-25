pub mod camera;
pub mod draw;
pub mod light;
pub mod model;
pub mod prelude;
pub mod shader;

use crate::{
    color::Color,
    math::vector::{Vector2, Vector3, Vector4},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex3D {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub diffuse: Color<u8>,
    pub specular: Color<u8>,
    pub uv: Vector2<f32>,
    pub subtexture: Vector2<f32>,
}

use dxlib_sys::data::Vertex3d as DxVertex3D;

impl From<Vertex3D> for DxVertex3D {
    fn from(v: Vertex3D) -> DxVertex3D {
        DxVertex3D {
            pos: v.position.into(),
            norm: v.normal.into(),
            dif: v.diffuse.into(),
            spc: v.specular.into(),
            u: v.uv[0],
            v: v.uv[1],
            su: v.subtexture[0],
            sv: v.subtexture[1],
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vertex3DShader {
    pub position: Vector3<f32>,
    pub subposition: Vector4<f32>,
    pub normal: Vector3<f32>,
    pub tangent: Vector3<f32>,
    pub binorm: Vector3<f32>,
    pub diffuse: Color<u8>,
    pub specular: Color<u8>,
    pub uv: Vector2<f32>,
    pub subtexture: Vector2<f32>,
}

use dxlib_sys::data::Vertex3dShader as DxVertex3DShader;

impl From<Vertex3DShader> for DxVertex3DShader {
    fn from(v: Vertex3DShader) -> DxVertex3DShader {
        DxVertex3DShader {
            pos: v.position.into(),
            spos: v.subposition.into(),
            norm: v.normal.into(),
            tan: v.tangent.into(),
            binorm: v.binorm.into(),
            dif: v.diffuse.into(),
            spc: v.specular.into(),
            u: v.uv[0],
            v: v.uv[1],
            su: v.subtexture[0],
            sv: v.subtexture[1],
        }
    }
}
