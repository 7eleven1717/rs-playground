use ffi::*;
use std::borrow::Cow;
use std::ffi::CStr;

pub enum Id {
    AV1,
    H264,
}

impl Id {
    pub fn get_name(&self) -> Cow<'static, str> {
        unsafe { CStr::from_ptr(avcodec_get_name(*self.as_ref())).to_string_lossy() }
    }
}

impl AsRef<AVCodecID> for Id {
    fn as_ref(&self) -> &AVCodecID {
        match self {
            Id::AV1 => &AVCodecID_AV_CODEC_ID_AV1,
            Id::H264 => &AVCodecID_AV_CODEC_ID_H264,
        }
    }
}
