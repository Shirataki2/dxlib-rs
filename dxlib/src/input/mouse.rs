use dxlib_sys::{
    consts::*, dx_GetMouseInput, dx_GetMouseInputLog2, dx_GetMousePoint, dx_GetMouseWheelRotVol,
    dx_SetMouseDispFlag, dx_SetMousePoint,
};

use crate::{
    error::{I32CodeExt, Result},
    math::vector::Vector2,
};
use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, IntoEnumIterator, FromPrimitive, PartialEq, Eq)]
#[repr(i32)]
pub enum MouseButton {
    Left = MOUSE_INPUT_LEFT,
    Right = MOUSE_INPUT_RIGHT,
    Middle = MOUSE_INPUT_MIDDLE,
    Four = MOUSE_INPUT_4,
    Five = MOUSE_INPUT_5,
    Six = MOUSE_INPUT_6,
    Seven = MOUSE_INPUT_7,
    Eight = MOUSE_INPUT_8,
}

#[derive(Debug, Clone, Copy, IntoEnumIterator, FromPrimitive)]
#[repr(i32)]
pub enum MouseInputType {
    Up = MOUSE_INPUT_LOG_UP,
    Down = MOUSE_INPUT_LOG_DOWN,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseInput {
    pub button: MouseButton,
    pub position: Vector2<i32>,
    pub input_type: MouseInputType,
}

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

    pub fn get_position_as_f32() -> Result<Vector2<f32>> {
        let mut x = 0;
        let mut y = 0;
        unsafe { dx_GetMousePoint(&mut x, &mut y).ensure_zero()? };
        Ok(Vector2::from([x as f32, y as f32]))
    }

    pub fn set_position(position: Vector2<i32>) -> Result<()> {
        unsafe { dx_SetMousePoint(position[0], position[1]).ensure_zero() }
    }

    pub fn get_button() -> Result<Vec<MouseButton>> {
        let bit = unsafe { dx_GetMouseInput() };
        Ok(MouseButton::into_enum_iter()
            .filter(|&flag| bit & flag as i32 > 0)
            .collect::<Vec<_>>())
    }

    pub fn get_input_log() -> Option<MouseInput> {
        let mut button = 0;
        let mut x = 0;
        let mut y = 0;
        let mut input_type = 0;
        let code = unsafe { dx_GetMouseInputLog2(&mut button, &mut x, &mut y, &mut input_type, 1) };
        if code == 0 {
            Some(MouseInput {
                button: num::FromPrimitive::from_i32(button)?,
                position: Vector2::from([x, y]),
                input_type: num::FromPrimitive::from_i32(input_type)?,
            })
        } else {
            None
        }
    }

    pub fn get_wheel_input(reset: bool) -> Result<i32> {
        let wheel = unsafe { dx_GetMouseWheelRotVol(reset as i32) };
        Ok(wheel)
    }
}
