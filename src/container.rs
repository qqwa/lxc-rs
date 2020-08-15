use lxc_sys2::lxc_container;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;

use crate::{DynResult, Error, Path};

pub struct Container {
    name: CString,
    internal_container: *mut lxc_container,
}

impl Container {
    /// Constructs a Container object. Note that the container does not get
    /// created and my be not yet defined. Use [Container.create] to
    /// actually create the container.
    pub fn new(name: &str, config_path: Path) -> DynResult<Self> {
        let name = CString::new(name)?;
        // let config_path = CString::new(config_path)?;

        let internal_container =
            unsafe { lxc_sys2::lxc_container_new(name.as_ptr(), config_path.get_cstr_pointer()) };

        if internal_container == std::ptr::null_mut() {
            Err(Error::ApiError("lxc_container_new".into()).into())
        } else {
            Ok(Container {
                name,
                internal_container,
            })
        }
    }

    pub(crate) fn from_raw_parts(name: *mut c_char, container: *mut lxc_container) -> Self {
        let name = unsafe { CString::from_raw(name) };
        Container {
            name,
            internal_container: container,
        }
    }

    pub fn get_config_path(&self) -> DynResult<&str> {
        Ok(unsafe { CStr::from_ptr((*self.internal_container).config_path) }.to_str()?)
    }

    /// Returns absolute path to config file of this container.
    /// This config file does not exist if [Container::is_defined] returns `false`.
    pub fn get_config_file_name(&self) -> DynResult<&str> {
        Ok(unsafe {
            CStr::from_ptr(((*self.internal_container).config_file_name)(
                self.internal_container,
            ))
        }
        .to_str()?)
    }

    pub fn is_running(&self) -> bool {
        unsafe { ((*self.internal_container).is_running)(self.internal_container) }
    }

    pub fn is_defined(&self) -> bool {
        unsafe { ((*self.internal_container).is_defined)(self.internal_container) }
    }

    pub fn create(&mut self) {
        todo!()
    }

    pub fn start(&mut self, useinit: i32) -> bool {
        unsafe { ((*self.internal_container).start)(self.internal_container, useinit, null_mut()) }
    }

    pub fn stop(&self) -> bool {
        unsafe { ((*self.internal_container).stop)(self.internal_container) }
    }
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.to_str().unwrap())
    }
}
