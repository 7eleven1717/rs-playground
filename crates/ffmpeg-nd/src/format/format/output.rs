mod functions;

use super::format::Format;
use crate::util::Error;
use core::ffi::CStr;
use core::ops::Deref;
use ffi::*;
pub use functions::*;
use std::borrow::Cow;

pub struct Output {
    ptr: *const AVOutputFormat,
}

impl Output {
    pub fn as_ptr(&self) -> *const AVOutputFormat {
        self.ptr
    }
}

impl Output {
    pub fn name(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.name).to_string_lossy() }
    }

    pub fn long_name(&self) -> Cow<'_, str> {
        unsafe { CStr::from_ptr(self.long_name).to_string_lossy() }
    }

    pub fn mime_type(&self) -> Option<Cow<'_, str>> {
        if self.mime_type.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(self.mime_type).to_string_lossy() })
        }
    }
}

impl Deref for Output {
    type Target = AVOutputFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl Default for Output {
    fn default() -> Self {
        Format::WebM.try_into().unwrap()
    }
}

impl From<*const AVOutputFormat> for Output {
    fn from(ptr: *const AVOutputFormat) -> Self {
        Output { ptr }
    }
}

impl TryFrom<Format> for Output {
    type Error = Error;

    fn try_from(format: Format) -> Result<Self, Self::Error> {
        if let Some(output) = guess_format(format.short_name(), "", format.mime_type()) {
            Ok(output)
        } else {
            Err("Could not find output file format".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use super::super::Format;
        let _output = Output::try_from(Format::default()).unwrap();
    }
}
