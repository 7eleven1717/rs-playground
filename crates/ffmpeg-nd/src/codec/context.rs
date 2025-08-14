mod video_context_builder;

use super::codec::Codec;
use super::id::Id;
use crate::util::{Error, PixelFormat, Rational};
use core::ops::{Deref, DerefMut};
use ffi::*;
pub use video_context_builder::VideoContextBuilder;

pub struct Context {
    ptr: *mut AVCodecContext,
}

impl Context {
    pub fn as_ptr(&self) -> *const AVCodecContext {
        self.ptr as *const _
    }
    pub fn as_mut_ptr(&mut self) -> *mut AVCodecContext {
        self.ptr
    }
}

impl Context {
    pub fn open(&mut self) -> Result<(), Error> {
        match unsafe { avcodec_open2(self.as_mut_ptr(), self.codec, core::ptr::null_mut()) } {
            e if e < 0 => Err(Error::from(e)),
            _ => Ok(()),
        }
    }

    pub fn set_width<T: Into<u16>>(&mut self, value: T) {
        self.width = value.into().into();
    }

    pub fn set_height<T: Into<u16>>(&mut self, value: T) {
        self.height = value.into().into();
    }

    pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        self.framerate = value.into().into();
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        self.time_base = value.into().into();
    }

    pub fn set_bit_rate<T: Into<u32>>(&mut self, value: T) {
        self.bit_rate = value.into().into();
    }

    pub fn set_gop_size<T: Into<u8>>(&mut self, value: T) {
        self.gop_size = value.into().into();
    }

    pub fn set_max_b_frames<T: Into<u8>>(&mut self, value: T) {
        self.max_b_frames = value.into().into();
    }

    pub fn pix_fmt(&self) -> PixelFormat {
        self.pix_fmt.into()
    }

    pub fn set_pix_fmt<T: Into<PixelFormat>>(&mut self, value: T) {
        self.pix_fmt = value.into().into();
    }
}

impl From<Codec> for Context {
    fn from(codec: Codec) -> Self {
        Context {
            ptr: unsafe { avcodec_alloc_context3(codec.as_ptr()) },
        }
    }
}

impl From<Id> for Context {
    fn from(id: Id) -> Self {
        id.into()
    }
}

impl Deref for Context {
    type Target = AVCodecContext;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { avcodec_free_context(&mut self.as_mut_ptr()) }
    }
}
