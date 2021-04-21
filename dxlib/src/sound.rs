use dxlib_sys::{
    consts::*, dx_ChangePanSoundMem, dx_ChangeVolumeSoundMem, dx_CheckSoundMem, dx_DeleteSoundMem,
    dx_LoadSoundMem, dx_PlaySoundMem, dx_SetFrequencySoundMem, dx_SetLoopPosSoundMem,
    dx_StopSoundMem,
};
use std::{path::Path, time::Duration};

use crate::{
    error::{DxLibError, I32CodeExt, Result},
    utils::path_to_cstring,
};

#[derive(Debug)]
pub struct Sound {
    handle: i32,
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum PlayType {
    Normal = DX_PLAYTYPE_NORMAL,
    Back = DX_PLAYTYPE_BACK,
    Loop = DX_PLAYTYPE_LOOP,
}

#[derive(Debug, Clone, Copy)]
pub enum Pan {
    Right(u8),
    Left(u8),
}

impl Sound {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Sound> {
        let path = path_to_cstring(&path)?;
        let handle = unsafe { dx_LoadSoundMem(path.as_ptr(), 3, -1).ensure_not_minus1()? };
        Ok(Self { handle })
    }

    pub fn play(&self, play_type: PlayType, play_from_top: bool) -> Result<()> {
        unsafe {
            dx_PlaySoundMem(self.handle, play_type as i32, play_from_top as i32).ensure_zero()
        }
    }

    pub fn is_playing(&self) -> Result<bool> {
        match unsafe { dx_CheckSoundMem(self.handle) } {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(DxLibError::NonZeroReturned),
        }
    }

    pub fn stop(&self) -> Result<()> {
        unsafe { dx_StopSoundMem(self.handle).ensure_zero() }
    }

    pub fn set_pan(&self, pan: Pan) -> Result<()> {
        let pan = match pan {
            Pan::Right(p) => p as i32,
            Pan::Left(p) => -(p as i32),
        };
        unsafe { dx_ChangePanSoundMem(pan, self.handle).ensure_zero() }
    }

    pub fn set_volume(&self, vol: u8) -> Result<()> {
        unsafe { dx_ChangeVolumeSoundMem(vol as i32, self.handle).ensure_zero() }
    }

    pub fn set_frequency(&self, freq: Option<i32>) -> Result<()> {
        let freq = match freq {
            Some(f) => {
                if !(100..=100000).contains(&f) {
                    return Err(DxLibError::InvalidRange(
                        String::from("Frequency"),
                        100,
                        100_000,
                    ));
                }
                f
            }
            None => -1,
        };

        unsafe { dx_SetFrequencySoundMem(freq, self.handle).ensure_zero() }
    }

    pub fn set_loop_position(&self, start_position: Duration) -> Result<()> {
        let msec = start_position.as_millis();
        if msec > std::i32::MAX as u128 {
            return Err(DxLibError::InvalidRange(
                String::from("StartPosition (unit: msec)"),
                0,
                std::i32::MAX,
            ));
        }
        unsafe { dx_SetLoopPosSoundMem(msec as i64, self.handle).ensure_zero() }
    }

    fn close(&self) -> Result<()> {
        unsafe { dx_DeleteSoundMem(self.handle, 0).ensure_zero() }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        let _ = self.close();
    }
}
