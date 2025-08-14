mod functions;

use super::context::VideoContextBuilder;
use core::ops::Deref;
use ffi::*;
pub use functions::*;

pub struct Codec {
    ptr: *const AVCodec,
}

impl Codec {
    pub fn as_ptr(&self) -> *const AVCodec {
        self.ptr as *const _
    }
}

impl Codec {
    /// Experimental
    pub fn into_video_context_builder(self) -> VideoContextBuilder {
        VideoContextBuilder::new(self)
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
