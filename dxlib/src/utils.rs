use encoding_rs::SHIFT_JIS;

pub fn to_sjis_cstring(s: &str) -> *const i8 {
    let s = format!("{}\0", s);
    let (bytes, _, _) = SHIFT_JIS.encode(&s);
    let bytes = bytes.as_ptr() as *const i8;
    bytes
}
