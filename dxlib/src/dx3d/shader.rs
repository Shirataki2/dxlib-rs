use std::path::Path;

use dxlib_sys::{
    dx_DeleteShader, dx_LoadPixelShader, dx_LoadVertexShader, dx_SetPSConstF, dx_SetPSConstFArray,
    dx_SetUsePixelShader, dx_SetUseVertexShader, dx_SetVSConstF, dx_SetVSConstFArray,
};

use crate::{
    error::{I32CodeExt, Result},
    math::vector::Vector4,
    utils::path_to_cstring,
};

#[derive(Debug)]
pub struct VertexShader {
    handle: i32,
}

impl VertexShader {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<VertexShader> {
        let path = path_to_cstring(&path)?;
        let handle = unsafe { dx_LoadVertexShader(path.as_ptr()).ensure_not_minus1()? };
        unsafe {
            dx_SetUseVertexShader(handle).ensure_zero()?;
        }
        Ok(VertexShader { handle })
    }

    pub fn set_float4_const(&mut self, index: u8, value: Vector4<f32>) -> Result<()> {
        unsafe { dx_SetVSConstF(index as i32, value.into()).ensure_zero() }
    }

    pub fn set_float4_const_list(&mut self, index: u8, value: &[Vector4<f32>]) -> Result<()> {
        use dxlib_sys::Float4 as DxFloat4;
        let value = value.iter().map(|v| DxFloat4::from(*v)).collect::<Vec<_>>();
        unsafe {
            dx_SetVSConstFArray(index as i32, value.as_ptr(), value.len() as i32).ensure_zero()
        }
    }
}

impl Drop for VertexShader {
    fn drop(&mut self) {
        unsafe { dx_DeleteShader(self.handle) };
    }
}

#[derive(Debug)]
pub struct PixelShader {
    handle: i32,
}

impl PixelShader {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<PixelShader> {
        let path = path_to_cstring(&path)?;
        let handle = unsafe { dx_LoadPixelShader(path.as_ptr()).ensure_not_minus1()? };
        unsafe {
            dx_SetUsePixelShader(handle).ensure_zero()?;
        }
        Ok(PixelShader { handle })
    }

    pub fn set_float4_const(&mut self, index: u8, value: Vector4<f32>) -> Result<()> {
        unsafe { dx_SetPSConstF(index as i32, value.into()).ensure_zero() }
    }

    pub fn set_float4_const_list(&mut self, index: u8, value: &[Vector4<f32>]) -> Result<()> {
        use dxlib_sys::Float4 as DxFloat4;
        let value = value.iter().map(|v| DxFloat4::from(*v)).collect::<Vec<_>>();
        unsafe {
            dx_SetPSConstFArray(index as i32, value.as_ptr(), value.len() as i32).ensure_zero()
        }
    }
}

impl Drop for PixelShader {
    fn drop(&mut self) {
        unsafe { dx_DeleteShader(self.handle) };
    }
}
