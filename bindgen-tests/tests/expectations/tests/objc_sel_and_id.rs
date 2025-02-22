#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target_os = "macos")]
use objc::{self, msg_send, sel, sel_impl, class};
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
unsafe extern "C" {
    pub static mut object: id;
}
unsafe extern "C" {
    pub static mut selector: objc::runtime::Sel;
}
unsafe extern "C" {
    pub fn f(object: id, selector: objc::runtime::Sel);
}
