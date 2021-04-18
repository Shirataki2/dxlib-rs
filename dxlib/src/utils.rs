use crate::error::{DxLibError, Result};
use dxlib_sys::ffi::dx_WaitKey;
use encoding_rs::SHIFT_JIS;
use std::{ffi::CString, path::Path};

pub fn to_sjis_cstring(s: &str) -> Vec<u8> {
    let s = format!("{}\0", s);
    let (bytes, _, _) = SHIFT_JIS.encode(&s);
    let bytes = bytes.to_vec();
    bytes
}

pub fn path_to_cstring<P: AsRef<Path>>(path: &P) -> Result<CString> {
    Ok(CString::new(
        path.as_ref().to_str().ok_or(DxLibError::InvalidPath)?,
    )?)
}

pub fn wait_any_key() -> Result<()> {
    unsafe {
        dx_WaitKey();
    }
    Ok(())
}
