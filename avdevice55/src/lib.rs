#![crate_name = "avdevice55"]
#![crate_type = "dylib"]
#![allow(non_camel_case_types)]
/* automatically generated by rust-bindgen */
extern crate libc;
extern crate avformat55 as avformat;
use libc::{size_t};

pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVDeviceRect {
    pub x: ::libc::c_int,
    pub y: ::libc::c_int,
    pub width: ::libc::c_int,
    pub height: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVDeviceRect {
    fn clone(&self) -> Struct_AVDeviceRect { *self }
}
impl ::std::default::Default for Struct_AVDeviceRect {
    fn default() -> Struct_AVDeviceRect { unsafe { ::std::mem::zeroed() } }
}
pub type AVDeviceRect = Struct_AVDeviceRect;
pub type Enum_AVAppToDevMessageType = ::libc::c_uint;
pub const AV_APP_TO_DEV_NONE: ::libc::c_uint = 1313820229;
pub const AV_APP_TO_DEV_WINDOW_SIZE: ::libc::c_uint = 1195724621;
pub const AV_APP_TO_DEV_WINDOW_REPAINT: ::libc::c_uint = 1380274241;
pub type Enum_AVDevToAppMessageType = ::libc::c_uint;
pub const AV_DEV_TO_APP_NONE: ::libc::c_uint = 1313820229;
pub const AV_DEV_TO_APP_CREATE_WINDOW_BUFFER: ::libc::c_uint = 1111708229;
pub const AV_DEV_TO_APP_PREPARE_WINDOW_BUFFER: ::libc::c_uint = 1112560197;
pub const AV_DEV_TO_APP_DISPLAY_WINDOW_BUFFER: ::libc::c_uint = 1111771475;
pub const AV_DEV_TO_APP_DESTROY_WINDOW_BUFFER: ::libc::c_uint = 1111770451;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVDeviceInfo {
    pub device_name: *mut ::libc::c_char,
    pub device_description: *mut ::libc::c_char,
}
impl ::std::clone::Clone for Struct_AVDeviceInfo {
    fn clone(&self) -> Struct_AVDeviceInfo { *self }
}
impl ::std::default::Default for Struct_AVDeviceInfo {
    fn default() -> Struct_AVDeviceInfo { unsafe { ::std::mem::zeroed() } }
}
pub type AVDeviceInfo = Struct_AVDeviceInfo;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_AVDeviceInfoList {
    pub devices: *mut *mut AVDeviceInfo,
    pub nb_devices: ::libc::c_int,
    pub default_device: ::libc::c_int,
}
impl ::std::clone::Clone for Struct_AVDeviceInfoList {
    fn clone(&self) -> Struct_AVDeviceInfoList { *self }
}
impl ::std::default::Default for Struct_AVDeviceInfoList {
    fn default() -> Struct_AVDeviceInfoList {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type AVDeviceInfoList = Struct_AVDeviceInfoList;
pub type __builtin_va_list = [__va_list_tag; 1usize];
pub type __va_list_tag = Struct___va_list_tag;
#[repr(C)]
#[derive(Copy)]
pub struct Struct___va_list_tag {
    pub gp_offset: ::libc::c_uint,
    pub fp_offset: ::libc::c_uint,
    pub overflow_arg_area: *mut ::libc::c_void,
    pub reg_save_area: *mut ::libc::c_void,
}
impl ::std::clone::Clone for Struct___va_list_tag {
    fn clone(&self) -> Struct___va_list_tag { *self }
}
impl ::std::default::Default for Struct___va_list_tag {
    fn default() -> Struct___va_list_tag { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "avdevice")]
extern "C" {
    pub fn avdevice_version() -> ::libc::c_uint;
    pub fn avdevice_configuration() -> *const ::libc::c_char;
    pub fn avdevice_license() -> *const ::libc::c_char;
    pub fn avdevice_register_all() -> ();
    pub fn avdevice_app_to_dev_control_message(s: *mut avformat::AVFormatContext,
                                               _type:
                                                   Enum_AVAppToDevMessageType,
                                               data: *mut ::libc::c_void,
                                               data_size: size_t)
     -> ::libc::c_int;
    pub fn avdevice_dev_to_app_control_message(s: *mut avformat::AVFormatContext,
                                               _type:
                                                   Enum_AVDevToAppMessageType,
                                               data: *mut ::libc::c_void,
                                               data_size: size_t)
     -> ::libc::c_int;
    pub fn avdevice_list_devices(s: *mut avformat::AVFormatContext,
                                 device_list: *mut *mut AVDeviceInfoList)
     -> ::libc::c_int;
    pub fn avdevice_free_list_devices(device_list: *mut *mut AVDeviceInfoList)
     -> ();
}

pub fn version() -> u32 {
    unsafe { avdevice_version() as u32 }
}

pub fn license() -> &'static str {
    std::str::from_utf8(unsafe { std::ffi::CStr::from_ptr(avdevice_license()) }.to_bytes()).ok().expect("invalid utf8")
}
