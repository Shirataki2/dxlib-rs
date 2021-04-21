use crate::error::{I32CodeExt, Result};
use dxlib_sys::{consts::*, dx_CheckHitKey, dx_GetHitKeyStateAll};
use enum_iterator::IntoEnumIterator;

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq, Eq)]
#[repr(i32)]
pub enum Key {
    A = KEY_INPUT_A,
    B = KEY_INPUT_B,
    C = KEY_INPUT_C,
    D = KEY_INPUT_D,
    E = KEY_INPUT_E,
    F = KEY_INPUT_F,
    G = KEY_INPUT_G,
    H = KEY_INPUT_H,
    I = KEY_INPUT_I,
    J = KEY_INPUT_J,
    K = KEY_INPUT_K,
    L = KEY_INPUT_L,
    M = KEY_INPUT_M,
    N = KEY_INPUT_N,
    O = KEY_INPUT_O,
    P = KEY_INPUT_P,
    Q = KEY_INPUT_Q,
    R = KEY_INPUT_R,
    S = KEY_INPUT_S,
    T = KEY_INPUT_T,
    U = KEY_INPUT_U,
    V = KEY_INPUT_V,
    W = KEY_INPUT_W,
    X = KEY_INPUT_X,
    Y = KEY_INPUT_Y,
    Z = KEY_INPUT_Z,
    ZERO = KEY_INPUT_0,
    ONE = KEY_INPUT_1,
    TWO = KEY_INPUT_2,
    THREE = KEY_INPUT_3,
    FOUR = KEY_INPUT_4,
    FIVE = KEY_INPUT_5,
    SIX = KEY_INPUT_6,
    SEVEN = KEY_INPUT_7,
    EIGHT = KEY_INPUT_8,
    NINE = KEY_INPUT_9,
    F1 = KEY_INPUT_F1,
    F2 = KEY_INPUT_F2,
    F3 = KEY_INPUT_F3,
    F4 = KEY_INPUT_F4,
    F5 = KEY_INPUT_F5,
    F6 = KEY_INPUT_F6,
    F7 = KEY_INPUT_F7,
    F8 = KEY_INPUT_F8,
    F9 = KEY_INPUT_F9,
    F10 = KEY_INPUT_F10,
    F11 = KEY_INPUT_F11,
    F12 = KEY_INPUT_F12,
    NUMPAD0 = KEY_INPUT_NUMPAD0,
    NUMPAD1 = KEY_INPUT_NUMPAD1,
    NUMPAD2 = KEY_INPUT_NUMPAD2,
    NUMPAD3 = KEY_INPUT_NUMPAD3,
    NUMPAD4 = KEY_INPUT_NUMPAD4,
    NUMPAD5 = KEY_INPUT_NUMPAD5,
    NUMPAD6 = KEY_INPUT_NUMPAD6,
    NUMPAD7 = KEY_INPUT_NUMPAD7,
    NUMPAD8 = KEY_INPUT_NUMPAD8,
    NUMPAD9 = KEY_INPUT_NUMPAD9,
    BACKSPACE = KEY_INPUT_BACK,
    TAB = KEY_INPUT_TAB,
    RETURN = KEY_INPUT_RETURN,
    LSHIFT = KEY_INPUT_LSHIFT,
    RSHIFT = KEY_INPUT_RSHIFT,
    LCONTROL = KEY_INPUT_LCONTROL,
    RCONTROL = KEY_INPUT_RCONTROL,
    ESCAPE = KEY_INPUT_ESCAPE,
    SPACE = KEY_INPUT_SPACE,
    PAGEUP = KEY_INPUT_PGUP,
    PAGEDOWN = KEY_INPUT_PGDN,
    END = KEY_INPUT_END,
    HOME = KEY_INPUT_HOME,
    LEFT = KEY_INPUT_LEFT,
    UP = KEY_INPUT_UP,
    RIGHT = KEY_INPUT_RIGHT,
    DOWN = KEY_INPUT_DOWN,
    INSERT = KEY_INPUT_INSERT,
    DELETE = KEY_INPUT_DELETE,
    MINUS = KEY_INPUT_MINUS,
    YEN = KEY_INPUT_YEN,
    PREVTRACK = KEY_INPUT_PREVTRACK,
    PERIOD = KEY_INPUT_PERIOD,
    SLASH = KEY_INPUT_SLASH,
    LALT = KEY_INPUT_LALT,
    RALT = KEY_INPUT_RALT,
    SCROLL = KEY_INPUT_SCROLL,
    SEMICOLON = KEY_INPUT_SEMICOLON,
    COLON = KEY_INPUT_COLON,
    LBRACKET = KEY_INPUT_LBRACKET,
    RBRACKET = KEY_INPUT_RBRACKET,
    AT = KEY_INPUT_AT,
    BACKSLASH = KEY_INPUT_BACKSLASH,
    COMMA = KEY_INPUT_COMMA,
    CAPSLOCK = KEY_INPUT_CAPSLOCK,
    PAUSE = KEY_INPUT_PAUSE,
}

pub struct KeyBoard;

impl KeyBoard {
    pub fn is_hit(key: Key) -> bool {
        unsafe { dx_CheckHitKey(key as i32) > 0 }
    }

    pub fn list_hit_keys() -> Result<Vec<Key>> {
        let mut keys = [0_i8; 256];
        unsafe {
            dx_GetHitKeyStateAll(&mut keys as *mut i8).ensure_zero()?;
        }
        let keys = Key::into_enum_iter()
            .filter(|key| keys[*key as usize] > 0)
            .collect::<Vec<_>>();
        Ok(keys)
    }
}
