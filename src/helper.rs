use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;

use crate::DynResult;
/// Specify whether to use the default path or provide a custom one.
pub enum Path {
    /// Use default
    Default,
    /// Use a custom path. Use [Path::custom] to construct this value.
    Custom(CString),
}

impl Path {
    pub fn custom(path: &str) -> DynResult<Self> {
        let string = CString::new(path)?;
        Ok(Path::Custom(string))
    }

    pub(crate) fn get_cstr_pointer(&self) -> *const c_char {
        match self {
            Path::Default => null_mut(),
            Path::Custom(lxcpath) => lxcpath.as_ptr(),
        }
    }
}
