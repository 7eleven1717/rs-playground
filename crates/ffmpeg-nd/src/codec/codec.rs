mod functions;
mod name;

use crate::util::MediaType;
use core::ffi::CStr;
use core::ops::Deref;
use ffi::*;
pub use functions::*;
pub use name::Name;
use std::borrow::Cow;

pub struct Codec {
    ptr: *const AVCodec,
}

impl Codec {
    pub fn as_ptr(&self) -> *const AVCodec {
        self.ptr as *const _
    }
}

impl Codec {
    pub fn name(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name).to_string_lossy() }
    }

    pub fn long_name(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.long_name).to_string_lossy() }
    }

    pub fn type_(&self) -> MediaType {
        self.type_.into()
    }

    pub fn is_encoder(&self) -> bool {
        unsafe { av_codec_is_encoder(self.ptr) != 0 }
    }

    pub fn is_decoder(&self) -> bool {
        unsafe { av_codec_is_decoder(self.ptr) != 0 }
    }
}

impl From<*const AVCodec> for Codec {
    fn from(ptr: *const AVCodec) -> Self {
        Codec { ptr }
    }
}

impl Deref for Codec {
    type Target = AVCodec;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
