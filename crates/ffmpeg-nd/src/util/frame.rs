use crate::codec::Context;
use crate::util::{Error, PixelFormat};
use core::ops::{Deref, DerefMut};
use core::ptr::null_mut;
use ffi::*;

pub struct Frame {
    ptr: *mut AVFrame,
}

impl Frame {
    pub fn as_ptr(&self) -> *const AVFrame {
        self.ptr as *const _
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVFrame {
        self.ptr
    }
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            ptr: unsafe { av_frame_alloc() },
        }
    }

    pub fn null() -> Self {
        Frame {
            ptr: null_mut(),
        }
    }

    pub fn get_buffer(&mut self) -> Result<(), Error> {
        match unsafe { av_frame_get_buffer(self.as_mut_ptr(), 0) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    #[inline(always)]
    pub fn make_writable(&mut self) -> Result<(), Error> {
        match unsafe { av_frame_make_writable(self.as_mut_ptr()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }
}

impl Frame {
    pub fn set_format(&mut self, value: PixelFormat) {
        self.format = value.into();
    }

    pub fn set_width(&mut self, value: u16) {
        self.width = value.into();
    }

    pub fn set_height(&mut self, value: u16) {
        self.height = value.into();
    }

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
    fn drop(&mut self) {
        unsafe { av_frame_free(&mut self.ptr) }
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

impl TryFrom<&Context> for Frame {
    type Error = Error;

    fn try_from(context: &Context) -> Result<Self, Self::Error> {
        let mut frame = Self::new();
        frame.set_format(context.pix_fmt());
        frame.set_width(context.width());
        frame.set_height(context.height());
        frame.get_buffer()?;
        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use crate::codec::Name;
        let context: Context = Name::default().default_video_context();
        Frame::try_from(&context).unwrap();
    }
}
