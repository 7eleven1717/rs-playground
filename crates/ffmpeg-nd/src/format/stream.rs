use super::super::codec::Context as CodecContext;
use super::context::Context;
use crate::util::{Error, Rational};
use ffi::*;
use std::ops::{Deref, DerefMut};

pub struct Stream {
    ptr: *mut AVStream,
}

impl Stream {
    pub fn new(context: &mut Context) -> Self {
        Stream {
            ptr: unsafe { avformat_new_stream(context.as_mut_ptr(), core::ptr::null_mut()) },
        }
    }

    pub fn parameters_from_context(&self, codec: &CodecContext) -> Result<(), Error> {
        match unsafe { avcodec_parameters_from_context(self.codecpar, codec.as_ptr()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    pub fn index(&self) -> u8 {
        self.index.try_into().unwrap()
    }

    pub fn time_base(&self) -> Rational {
        self.time_base.into()
    }
}

impl<'a> Deref for Stream {
    type Target = AVStream;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<'a> DerefMut for Stream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use crate::codec::Name;
        use crate::format::Context;

        let stream = Stream::new(&mut Context::default());
        stream
            .parameters_from_context(&Name::default().default_video_context())
            .unwrap();
    }
}
