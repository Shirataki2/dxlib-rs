use std::mem;

use dxlib_sys::{
    consts::*, dx_GetJoypadAnalogInput, dx_GetJoypadDirectInputState, dx_GetJoypadInputState,
};
use enum_iterator::IntoEnumIterator;

use crate::{
    error::{I32CodeExt, Result},
    math::vector::{Vector2, Vector3},
};

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
#[repr(i32)]
pub enum JoypadKey {
    Down = PAD_INPUT_DOWN,
    Left = PAD_INPUT_LEFT,
    Right = PAD_INPUT_RIGHT,
    Up = PAD_INPUT_UP,
    /// Z key
    Key1 = PAD_INPUT_1,
    /// X key
    Key2 = PAD_INPUT_2,
    /// C key
    Key3 = PAD_INPUT_3,
    /// A key
    Key4 = PAD_INPUT_4,
    /// S key
    Key5 = PAD_INPUT_5,
    /// D key
    Key6 = PAD_INPUT_6,
    /// Q key
    Key7 = PAD_INPUT_7,
    /// W key
    Key8 = PAD_INPUT_8,
    /// ESC key
    Key9 = PAD_INPUT_9,
    /// Space key
    Key10 = PAD_INPUT_10,
    Key11 = PAD_INPUT_11,
    Key12 = PAD_INPUT_12,
    Key13 = PAD_INPUT_13,
    Key14 = PAD_INPUT_14,
    Key15 = PAD_INPUT_15,
    Key16 = PAD_INPUT_16,
    Key17 = PAD_INPUT_17,
    Key18 = PAD_INPUT_18,
    Key19 = PAD_INPUT_19,
    Key20 = PAD_INPUT_20,
    Key21 = PAD_INPUT_21,
    Key22 = PAD_INPUT_22,
    Key23 = PAD_INPUT_23,
    Key24 = PAD_INPUT_24,
    Key25 = PAD_INPUT_25,
    Key26 = PAD_INPUT_26,
    Key27 = PAD_INPUT_27,
    Key28 = PAD_INPUT_28,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum JoypadType {
    Pad1 = DX_INPUT_PAD1,
    Pad2 = DX_INPUT_PAD2,
    Pad3 = DX_INPUT_PAD3,
    Pad4 = DX_INPUT_PAD4,
}

use dxlib_sys::data::DInputJoyState as DxDInputJoyState;

#[derive(Debug)]
pub struct DInputJoyState {
    pub stick: Vector3<i32>,
    pub rotate: Vector3<i32>,
    pub slider: [i32; 2],
    pub pov: [u32; 4],
    pub buttons: [u8; 32],
}

impl From<DxDInputJoyState> for DInputJoyState {
    fn from(state: DxDInputJoyState) -> DInputJoyState {
        DInputJoyState {
            stick: Vector3::from([state.x, state.y, state.z]),
            rotate: Vector3::from([state.rx, state.ry, state.rz]),
            slider: state.slider,
            pov: state.pov,
            buttons: state.buttons,
        }
    }
}

#[derive(Debug)]
pub struct Joypad;

impl Joypad {
    pub fn get_keypad_input() -> Vec<JoypadKey> {
        let state = unsafe { dx_GetJoypadInputState(DX_INPUT_KEY_PAD1) };
        JoypadKey::into_enum_iter()
            .filter(|&key| (key as i32) & state > 0)
            .collect::<Vec<_>>()
    }

    pub fn get_key_input() -> Vec<JoypadKey> {
        let state = unsafe { dx_GetJoypadInputState(DX_INPUT_KEY) };
        JoypadKey::into_enum_iter()
            .filter(|&key| (key as i32) & state > 0)
            .collect::<Vec<_>>()
    }

    pub fn get_pad_input(pad: JoypadType) -> Vec<JoypadKey> {
        let state = unsafe { dx_GetJoypadInputState(pad as i32) };
        JoypadKey::into_enum_iter()
            .filter(|&key| (key as i32) & state > 0)
            .collect::<Vec<_>>()
    }

    pub fn get_analog_input(pad: JoypadType) -> Result<Vector2<i32>> {
        let (mut x, mut y) = (0, 0);
        unsafe {
            dx_GetJoypadAnalogInput(&mut x, &mut y, pad as i32).ensure_zero()?;
        }
        Ok(Vector2::from([x, y]))
    }

    pub fn get_direct_input(pad: JoypadType) -> Result<DInputJoyState> {
        let mut dinput: DxDInputJoyState = unsafe { mem::zeroed() };
        unsafe {
            dx_GetJoypadDirectInputState(pad as i32, &mut dinput).ensure_zero()?;
        }
        Ok(DInputJoyState::from(dinput))
    }
}
