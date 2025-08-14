use ffi::*;
use std::borrow::Cow;
use std::ffi::CStr;

pub fn version_info() -> Option<Cow<'static, str>> {
    let ptr = unsafe { av_version_info() };
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(ptr).to_string_lossy() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        let version = version_info().unwrap();
        assert_eq!(&*version, "7.1.1");
    }
}
