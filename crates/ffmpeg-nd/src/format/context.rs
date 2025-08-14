use super::format::{Format, Output};
use super::io::{Context as IOContext, Flag};
use crate::codec::Packet;
use crate::util::Error;
use ffi::*;
use std::ffi::CString;

pub struct Context {
    ptr: *mut AVFormatContext,
    _pb: Option<IOContext>,
}

impl Context {
    pub fn as_ptr(&self) -> *const AVFormatContext {
        self.ptr
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVFormatContext {
        self.ptr
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            ptr: unsafe { avformat_alloc_context() },
            _pb: None,
        }
    }

    pub fn alloc_output(&mut self, output: Output) -> Result<(), Error> {
        match unsafe {
            avformat_alloc_output_context2(
                &mut self.ptr,
                output.as_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    /// Set private byte IO buffer
    pub fn set_pb(&mut self, mut pb: IOContext) {
        unsafe { (*self.ptr).pb = pb.as_mut_ptr() }
        self._pb = Some(pb);
    }

    pub fn open_read<U: AsRef<str>>(&mut self, url: U) -> Result<(), Error> {
        let mut pb = IOContext::from(unsafe { (*self.ptr).pb });
        pb.open(url, Flag::Read)?;
        self.set_pb(pb);
        Ok(())
    }

    pub fn open_write<U: AsRef<str>>(&mut self, url: U) -> Result<(), Error> {
        let mut pb = IOContext::from(unsafe { (*self.ptr).pb });
        pb.open(url, Flag::Write)?;
        self.set_pb(pb);
        Ok(())
    }

    pub fn write_header(&mut self) -> Result<(), Error> {
        match unsafe { avformat_write_header(self.ptr, core::ptr::null_mut()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    pub fn write_trailer(&mut self) -> Result<(), Error> {
        match unsafe { av_write_trailer(self.ptr) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    pub fn interleaved_write_frame(&mut self, pkt: &mut Packet) -> Result<(), Error> {
        match unsafe { av_interleaved_write_frame(self.ptr, pkt.as_mut_ptr()) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }

    pub fn dump_format(&mut self, index: i8, url: Option<&str>, is_output: bool) {
        unsafe {
            av_dump_format(
                self.ptr,
                index.into(),
                CString::new(url.unwrap_or("")).unwrap().as_ptr(),
                is_output.into(),
            )
        };
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { avformat_free_context(self.ptr) };
    }
}

impl TryFrom<Format> for Context {
    type Error = Error;

    fn try_from(format: Format) -> Result<Self, Self::Error> {
        Output::try_from(format)?.try_into()
    }
}

impl TryFrom<Output> for Context {
    type Error = Error;

    fn try_from(output: Output) -> Result<Self, Error> {
        let mut context = Self::new();
        context.alloc_output(output)?;
        Ok(context)
    }
}

impl Default for Context {
    fn default() -> Self {
        Format::default().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut context = Context::default();
        context.dump_format(0, None, true);
    }
}
