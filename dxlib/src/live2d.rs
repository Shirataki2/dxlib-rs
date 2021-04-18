use dxlib_sys::{
    dx_Live2D_DeleteModel, dx_Live2D_LoadModel, dx_Live2D_Model_Draw,
    dx_Live2D_Model_IsMotionFinished, dx_Live2D_Model_SetExtendRate, dx_Live2D_Model_SetTranslate,
    dx_Live2D_Model_StartMotion, dx_Live2D_Model_Update, dx_Live2D_RenderBegin,
    dx_Live2D_RenderEnd, dx_Live2D_SetCubism4CoreDLLPath,
};
use std::{collections::BTreeMap, ffi::CString, path::Path, time::Duration};

use crate::{
    error::{DxLibError, I32CodeExt, Result},
    math::vector::Vector2,
};

pub struct Live2DRenderer<'model> {
    models: BTreeMap<i32, &'model Live2DModel>,
}

impl<'model> Live2DRenderer<'model> {
    pub fn new<P: AsRef<Path>>(dll_path: P) -> Result<Self> {
        let dll_path = CString::new(dll_path.as_ref().to_str().ok_or(DxLibError::InvalidPath)?)?;
        unsafe { dx_Live2D_SetCubism4CoreDLLPath(dll_path.as_ptr()).ensure_zero()? };
        let models = BTreeMap::new();
        Ok(Live2DRenderer { models })
    }

    pub fn add_model(&mut self, model: &'model Live2DModel) -> Result<()> {
        self.models.insert(model.handle, &model);
        Ok(())
    }

    pub fn remove_model(&mut self, model: Live2DModel) -> Result<()> {
        self.models.remove(&model.handle);
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        unsafe {
            dx_Live2D_RenderBegin().ensure_zero()?;
            for model in self.models.iter() {
                dx_Live2D_Model_Draw(*model.0).ensure_zero()?;
            }
            dx_Live2D_RenderEnd().ensure_zero()?;
        }
        Ok(())
    }
}

pub struct Live2DModel {
    handle: i32,
}

impl Live2DModel {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Live2DModel> {
        let path = CString::new(path.as_ref().to_str().ok_or(DxLibError::InvalidPath)?)?;
        let handle = unsafe { dx_Live2D_LoadModel(path.as_ptr()).ensure_not_minus1()? };
        Ok(Live2DModel { handle })
    }

    pub fn is_motion_finished(&self) -> Result<bool> {
        unsafe { Ok(dx_Live2D_Model_IsMotionFinished(self.handle).ensure_positive()? == 1) }
    }

    pub fn start_motion(&self, group: &str, index: usize) -> Result<()> {
        let group = CString::new(group.as_bytes())?;
        unsafe {
            dx_Live2D_Model_StartMotion(self.handle, group.as_ptr(), index as i32).ensure_zero()?
        };
        Ok(())
    }

    pub fn update(&self, duration: Duration) -> Result<()> {
        let secs = duration.as_secs_f32();
        unsafe {
            dx_Live2D_Model_Update(self.handle, secs).ensure_zero()?;
        }
        Ok(())
    }

    pub fn translate(&self, v: Vector2<f32>) -> Result<()> {
        unsafe { dx_Live2D_Model_SetTranslate(self.handle, v[0], v[1]).ensure_zero() }
    }

    pub fn scale(&self, v: Vector2<f32>) -> Result<()> {
        unsafe { dx_Live2D_Model_SetExtendRate(self.handle, v[0], v[1]).ensure_zero() }
    }

    fn remove(&self) -> Result<()> {
        unsafe { dx_Live2D_DeleteModel(self.handle).ensure_zero() }
    }
}

impl Drop for Live2DModel {
    fn drop(&mut self) {
        let _ = self.remove();
    }
}
