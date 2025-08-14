use crate::util::Rational;
use core::ops::{Deref, DerefMut};
use core::slice;
use ffi::*;

#[derive(Clone)]
pub struct Packet {
    ptr: *mut AVPacket,
}

impl Packet {
    pub fn as_ptr(&self) -> *const AVPacket {
        self.ptr as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVPacket {
        self.ptr
    }
}

impl Packet {
    pub fn new() -> Self {
        Packet {
            ptr: unsafe { av_packet_alloc() },
        }
    }

    #[inline(always)]
    pub fn unref(&mut self) {
        unsafe { av_packet_unref(self.ptr) }
    }

    #[inline(always)]
    pub fn data(&mut self) -> Option<&[u8]> {
        if self.data.is_null() {
            None
        } else {
            Some(unsafe { slice::from_raw_parts(self.data, self.size as usize) })
        }
    }

    #[inline(always)]
    pub fn set_stream_index(&mut self, index: u8) {
        self.stream_index = index.into()
    }

    #[inline(always)]
    pub fn rescale_ts(&mut self, tb_src: &Rational, tb_dst: &Rational) {
        unsafe { av_packet_rescale_ts(self.ptr, tb_src.into(), tb_dst.into()) };
    }
}

impl Deref for Packet {
    type Target = AVPacket;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

impl Drop for Packet {
    fn drop(&mut self) {
        unsafe { av_packet_free(&mut self.ptr) }
    }
}
