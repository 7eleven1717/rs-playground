use crate::codec::{codec_is_encoder, codec_iterate};
use ffi::*;
use std::borrow::Cow;
use std::ffi::CStr;

pub fn version_info() -> Option<Cow<'static, str>> {
    let ptr = unsafe { av_version_info() };
    if ptr.is_null() {
        return None;
    }
    let version = unsafe { CStr::from_ptr(ptr).to_string_lossy() };
    Some(version)
}

pub fn available_video_encoders() -> impl Iterator<Item = (Cow<'static, str>, Cow<'static, str>)> {
    codec_iterate()
        .filter(|codec| {
            codec_is_encoder(codec) && codec.type_ == AVMediaType_AVMEDIA_TYPE_VIDEO
        })
        .map(|codec| {
            let codec_name = unsafe { CStr::from_ptr(codec.name).to_string_lossy() };
            let codec_long_name = unsafe { CStr::from_ptr(codec.long_name).to_string_lossy() };
            (codec_name, codec_long_name)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        let version = version_info().unwrap();
        assert_eq!(&*version, "7.1.1");
    }

    #[test]
    fn test_available_video_encoders() {
        for (name, long_name) in available_video_encoders() {
            println!("{}: {}", name, long_name);
        }
    }
}
