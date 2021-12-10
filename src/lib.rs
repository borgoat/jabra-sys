#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use std::os::raw::{c_char, c_int};

    use super::*;

    #[test]
    fn check_sdk_version() {
        const BUF_SIZE: usize = 32;
        let mut buffer = vec![0u8; BUF_SIZE];
        let count = (BUF_SIZE - 1) as c_int;
        let version = unsafe {
            let ret_code = Jabra_GetVersion(buffer.as_mut_ptr() as *mut c_char, count);
            assert_eq!(ret_code, _ReturnCode_Return_Ok);
            CStr::from_ptr(buffer.as_ptr() as *const c_char).to_str().unwrap()
        };
        assert_eq!(version, "1.10.1.0");
    }
}
