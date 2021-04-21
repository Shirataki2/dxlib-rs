use dxlib_sys::{
    dx_Live2D_DeleteModel, dx_Live2D_LoadModel, dx_Live2D_Model_Draw,
    dx_Live2D_Model_GetHitAreaName, dx_Live2D_Model_GetHitAreasCount,
    dx_Live2D_Model_GetParameterValue, dx_Live2D_Model_HitTest, dx_Live2D_Model_IsMotionFinished,
    dx_Live2D_Model_SetExpression, dx_Live2D_Model_SetExtendRate,
    dx_Live2D_Model_SetParameterValue, dx_Live2D_Model_SetRotate, dx_Live2D_Model_SetTranslate,
    dx_Live2D_Model_StartMotion, dx_Live2D_Model_Update, dx_Live2D_RenderBegin,
    dx_Live2D_RenderEnd, dx_Live2D_SetCubism4CoreDLLPath,
};
use std::{collections::BTreeMap, ffi::{CStr, CString}, path::Path, time::Duration};

use crate::{
    error::{DxLibError, I32CodeExt, Result},
    math::{angle::Angle, vector::Vector2},
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

#[derive(Debug)]
pub struct Live2DModel {
    handle: i32,
}

impl Live2DModel {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Live2DModel> {
        let path = CString::new(path.as_ref().to_str().ok_or(DxLibError::InvalidPath)?)?;
        let handle = unsafe { dx_Live2D_LoadModel(path.as_ptr()).ensure_not_minus1()? };
        Ok(Live2DModel { handle })
    }

    pub fn start_motion(&self, group: &str, index: usize) -> Result<()> {
        let group = CString::new(group.as_bytes())?;
        unsafe {
            dx_Live2D_Model_StartMotion(self.handle, group.as_ptr(), index as i32).ensure_zero()?
        };
        Ok(())
    }

    pub fn is_motion_finished(&self) -> Result<bool> {
        match unsafe { dx_Live2D_Model_IsMotionFinished(self.handle) } {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(DxLibError::NonZeroReturned),
        }
    }

    pub fn set_expression(&self, expression: &str) -> Result<()> {
        let expression = CString::new(expression.as_bytes())?;
        unsafe { dx_Live2D_Model_SetExpression(self.handle, expression.as_ptr()).ensure_zero() }
    }

    pub fn get_parameter(&self, name: &str) -> Result<f32> {
        let name = CString::new(name.as_bytes())?;
        let code = unsafe { dx_Live2D_Model_GetParameterValue(self.handle, name.as_ptr()) };
        if (code - 1.0f32).abs() < std::f32::EPSILON {
            Err(DxLibError::NonZeroReturned)
        } else {
            Ok(code)
        }
    }

    pub fn set_parameter(&self, name: &str, value: f32) -> Result<()> {
        let name = CString::new(name.as_bytes())?;
        unsafe {
            dx_Live2D_Model_SetParameterValue(self.handle, name.as_ptr(), value).ensure_zero()
        }
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

    pub fn rotate(&self, angle: Angle<f32>) -> Result<()> {
        unsafe { dx_Live2D_Model_SetRotate(self.handle, angle.to_radians()).ensure_zero() }
    }

    pub fn get_hitarea_count(&self) -> Result<usize> {
        unsafe {
            dx_Live2D_Model_GetHitAreasCount(self.handle)
                .ensure_positive().map(|r| r as usize)
        }
    }

    pub fn get_hitarea_name(&self, index: usize) -> Result<String> {
        let name = unsafe { dx_Live2D_Model_GetHitAreaName(self.handle, index as i32) };
        if name.is_null() {
            return Err(DxLibError::NullReturned);
        }
        let name = unsafe { CStr::from_ptr(name) };
        let name = name.to_string_lossy();
        Ok(name.to_string())
    }

    pub fn list_hitareas(&self) -> Result<Vec<String>> {
        let count = self.get_hitarea_count()?;
        (0..count)
            .map(|i| self.get_hitarea_name(i))
            .collect::<Result<Vec<_>>>()
    }

    pub fn is_hit(&self, hitarea_name: &str, coord: &Vector2<f32>) -> Result<bool> {
        let name = CString::new(hitarea_name.as_bytes())?;
        Ok(unsafe {
            dx_Live2D_Model_HitTest(self.handle, name.as_ptr(), coord[0], coord[1])
                .ensure_positive()?
                == 1
        })
    }

    pub fn list_hits(&self, coord: &Vector2<f32>) -> Result<Vec<String>> {
        let names = self
            .list_hitareas()?
            .iter()
            .filter_map(|area| match self.is_hit(area, coord) { 
                Ok(true) => Some(area.clone()),
                _ => None,
             })
            .collect::<Vec<_>>();
        Ok(names)
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
