use dxlib_sys::{dx_GetMousePoint, dx_SetMouseDispFlag, dx_SetMousePoint};

use crate::{error::{I32CodeExt, Result}, math::vector::Vector2};

pub struct Mouse;

impl Mouse {
    pub fn set_visible(visible: bool) -> Result<()> {
        unsafe { dx_SetMouseDispFlag(if visible { 1 } else { 0 }).ensure_zero() }
    }

    pub fn get_position() -> Result<Vector2<i32>> {
        let mut x = 0;
        let mut y = 0;
        unsafe { dx_GetMousePoint(&mut x, &mut y).ensure_zero()? };
        Ok(Vector2::from([x, y]))
    }

    pub fn set_position(position: Vector2<i32>) -> Result<()> {
        unsafe { dx_SetMousePoint(position[0], position[1]).ensure_zero() }
    }

}