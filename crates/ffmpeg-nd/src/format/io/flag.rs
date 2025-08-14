use ffi::*;
use std::os::raw::c_int;

pub enum Flag {
    Read,
    Write,
    ReadWrite,
}

impl Into<c_int> for Flag {
    fn into(self) -> c_int {
        match self {
            Flag::Read => AVIO_FLAG_READ,
            Flag::Write => AVIO_FLAG_WRITE,
            Flag::ReadWrite => AVIO_FLAG_READ_WRITE,
        }
    }
}
