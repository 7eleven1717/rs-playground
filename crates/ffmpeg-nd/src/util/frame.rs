use crate::util::{Error, PixelFormat};
use core::ops::{Deref, DerefMut};
use core::ptr;
use ffi::*;

pub struct Frame {
    ptr: *mut AVFrame,
}

impl Frame {
    #[inline(always)]
    pub fn as_ptr(&self) -> *const AVFrame {
        self.ptr as *const _
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut AVFrame {
        self.ptr
    }
}

impl Frame {
    #[inline(always)]
    pub fn new() -> Self {
        Frame {
            ptr: unsafe { av_frame_alloc() },
        }
    }

    #[inline(always)]
    pub fn null() -> Self {
        Frame {
            ptr: ptr::null_mut(),
        }
    }

    #[inline(always)]
    pub fn get_buffer(&mut self) -> Result<(), Error> {
        match unsafe { av_frame_get_buffer(self.as_mut_ptr(), 0) } {
            e if e < 0 => Err(Error::from(e)),
            _ => Ok(()),
        }
    }

    #[inline(always)]
    pub fn make_writable(&mut self) -> Result<(), Error> {
        match unsafe { av_frame_make_writable(self.as_mut_ptr()) } {
            e if e < 0 => Err(Error::from(e)),
            _ => Ok(()),
        }
    }
}

impl Frame {
    #[inline]
    pub fn set_format(&mut self, value: PixelFormat) {
        self.format = value.into();
    }

    #[inline]
    pub fn set_width(&mut self, value: u16) {
        self.width = value.into();
    }

    #[inline]
    pub fn set_height(&mut self, value: u16) {
        self.height = value.into();
    }

    #[inline]
    pub fn set_pts(&mut self, value: u32) {
        self.pts = value.into();
    }
}

impl Frame {
    /// Utility function to get the num of pixels in the frame
    pub fn pixel_count(&self) -> usize {
        self.width as usize * self.height as usize
    }
}

impl Drop for Frame {
    #[inline]
    fn drop(&mut self) {
        unsafe { av_frame_free(&mut self.as_mut_ptr()) }
    }
}

impl Deref for Frame {
    type Target = AVFrame;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl DerefMut for Frame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

use crate::codec::Context;
impl TryFrom<&Context> for Frame {
    type Error = Error;

    fn try_from(context: &Context) -> Result<Self, Self::Error> {
        let mut frame = Self::new();
        unsafe {
            let mut_ptr = frame.as_mut_ptr();
            (*mut_ptr).format = context.pix_fmt;
            (*mut_ptr).width = context.width;
            (*mut_ptr).height = context.height;
        };
        frame.get_buffer()?;
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use crate::codec::Id;
        let context: Context = Id::AV1
            .find_encoder()
            .unwrap()
            .into_video_context_builder()
            .framerate(30)
            .time_base([1, 30])
            .width(1280)
            .height(720)
            .bit_rate(400000)
            .gop_size(60)
            .pix_fmt(PixelFormat::YUV444P)
            .build()
            .unwrap();
        Frame::try_from(&context).unwrap();
    }
}
