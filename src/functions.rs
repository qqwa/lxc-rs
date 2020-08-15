use std::ffi::CStr;
use std::ptr::null_mut;

use crate::{Container, Error, LxcResult, Path};

pub fn get_version() -> &'static str {
    unsafe { CStr::from_ptr(lxc_sys2::lxc_get_version()) }
        .to_str()
        .unwrap()
}

pub fn list_all_containers(lxcpath: Path) -> LxcResult<Vec<Container>> {
    let mut names_ptr: *mut *mut i8 = null_mut();
    let mut cret_ptr: *mut *mut lxc_sys2::lxc_container = null_mut();

    let ret = unsafe {
        lxc_sys2::list_all_containers(lxcpath.get_cstr_pointer(), &mut names_ptr, &mut cret_ptr)
    };
    if ret == -1 {
        return Err(Error::ApiError("list_all_containers".into()).into());
    }
    let count = ret as usize;
    let names = unsafe { std::slice::from_raw_parts_mut(names_ptr, count) };
    let cret = unsafe { std::slice::from_raw_parts_mut(cret_ptr, count) };

    let mut result = Vec::with_capacity(count);
    for (name, cret) in names[..].iter().zip(cret[..].iter()) {
        if *name != null_mut() && *cret != null_mut() {
            result.push(Container::from_raw_parts(*name, *cret));
        }
    }
    Ok(result)
}

pub fn list_active_containers(lxcpath: Path) -> LxcResult<Vec<Container>> {
    let mut names_ptr: *mut *mut i8 = null_mut();
    let mut cret_ptr: *mut *mut lxc_sys2::lxc_container = null_mut();

    let ret = unsafe {
        lxc_sys2::list_active_containers(lxcpath.get_cstr_pointer(), &mut names_ptr, &mut cret_ptr)
    };
    if ret == -1 {
        return Err(Error::ApiError("list_active_containers".into()).into());
    }
    let count = ret as usize;
    let names = unsafe { std::slice::from_raw_parts_mut(names_ptr, count) };
    let cret = unsafe { std::slice::from_raw_parts_mut(cret_ptr, count) };

    let mut result = Vec::with_capacity(count);
    for (name, cret) in names[..].iter().zip(cret[..].iter()) {
        if *name != null_mut() && *cret != null_mut() {
            result.push(Container::from_raw_parts(*name, *cret));
        }
    }
    Ok(result)
}

pub fn list_defined_containers(lxcpath: Path) -> LxcResult<Vec<Container>> {
    let mut names_ptr: *mut *mut i8 = null_mut();
    let mut cret_ptr: *mut *mut lxc_sys2::lxc_container = null_mut();

    let ret = unsafe {
        lxc_sys2::list_defined_containers(lxcpath.get_cstr_pointer(), &mut names_ptr, &mut cret_ptr)
    };
    if ret == -1 {
        return Err(Error::ApiError("list_defined_containers".into()).into());
    }
    let count = ret as usize;
    let names = unsafe { std::slice::from_raw_parts_mut(names_ptr, count) };
    let cret = unsafe { std::slice::from_raw_parts_mut(cret_ptr, count) };

    let mut result = Vec::with_capacity(count);
    for (name, cret) in names[..].iter().zip(cret[..].iter()) {
        if *name != null_mut() && *cret != null_mut() {
            result.push(Container::from_raw_parts(*name, *cret));
        }
    }
    Ok(result)
}
