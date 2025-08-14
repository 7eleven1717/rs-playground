use super::codec::Codec;
use super::packet::Packet;
use crate::util::{Error, Frame, MediaType, PixelFormat, Rational, SearchFlag};
use core::ops::{Deref, DerefMut};
use ffi::*;
use std::ffi::CString;

#[derive(Default)]
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
    #[inline(always)]
    pub fn send_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        match unsafe { avcodec_send_frame(self.ptr, frame.as_ptr()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    #[inline(always)]
    pub fn send_eof(&mut self) -> Result<(), Error> {
        self.send_frame(&Frame::null())
    }

    #[inline(always)]
    pub fn receive_packet(&mut self, packet: &mut Packet) -> Result<(), Error> {
        match unsafe { avcodec_receive_packet(self.ptr, packet.as_mut_ptr()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    #[inline(always)]
    pub fn codec_type(&self) -> MediaType {
        self.codec_type.into()
    }

    #[inline(always)]
    pub fn frame_num(&self) -> u32 {
        self.frame_num.try_into().unwrap()
    }
}

impl Context {
    pub fn open(&mut self) -> Result<(), Error> {
        match unsafe { avcodec_open2(self.ptr, self.codec, core::ptr::null_mut()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    pub fn width(&self) -> u16 {
        self.width.try_into().unwrap()
    }

    pub fn set_width(&mut self, value: u16) {
        self.width = value.into();
    }

    pub fn height(&self) -> u16 {
        self.height.try_into().unwrap()
    }

    pub fn set_height(&mut self, value: u16) {
        self.height = value.into();
    }

    pub fn set_frame_rate<R: Into<Rational>>(&mut self, value: R) {
        self.framerate = value.into().into();
    }

    pub fn time_base(&self) -> Rational {
        self.time_base.into()
    }

    pub fn set_time_base<R: Into<Rational>>(&mut self, value: R) {
        self.time_base = value.into().into();
    }

    pub fn set_bit_rate(&mut self, value: u32) {
        self.bit_rate = value.into();
    }

    pub fn set_gop_size(&mut self, value: u8) {
        self.gop_size = value.into();
    }

    pub fn set_max_b_frames(&mut self, value: u8) {
        self.max_b_frames = value.into();
    }

    pub fn pix_fmt(&self) -> PixelFormat {
        self.pix_fmt.into()
    }

    pub fn set_pix_fmt<T: Into<PixelFormat>>(&mut self, value: T) {
        self.pix_fmt = value.into().into();
    }

    pub fn set_opt(&mut self, name: &str, val: &str, flag: SearchFlag) -> Result<(), Error> {
        let n = CString::new(name).unwrap_or_default();
        let v = CString::new(val).unwrap_or_default();
        match unsafe { av_opt_set(self.priv_data, n.as_ptr(), v.as_ptr(), flag.into()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
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
        unsafe { avcodec_free_context(&mut self.ptr) }
    }
}

impl From<Codec> for Context {
    fn from(codec: Codec) -> Self {
        Context {
            ptr: unsafe { avcodec_alloc_context3(codec.as_ptr()) },
        }
    }
}
