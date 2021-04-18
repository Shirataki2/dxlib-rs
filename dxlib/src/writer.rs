use dxlib_sys::{dx_clsDx, dx_printfDx};
use std::fmt::{self, Error};

use crate::{
    error::{I32CodeExt, Result as DxResult},
    utils::to_sjis_cstring,
};
#[derive(Debug, Default)]
pub struct DebugWriter;

impl DebugWriter {
    pub fn clear(&mut self) -> DxResult<()> {
        unsafe { dx_clsDx().ensure_zero() }
    }
}

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        let s = to_sjis_cstring(s);
        let s = s.as_ptr() as *const i8;
        let result = unsafe { dx_printfDx(s).ensure_zero() };
        if result.is_err() {
            return Err(Error::default());
        }
        Ok(())
    }
}
