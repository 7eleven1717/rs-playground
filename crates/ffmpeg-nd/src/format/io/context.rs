use super::flag::Flag;
use crate::util::Error;
use ffi::*;
use std::ffi::CString;
use url::Url;

pub struct Context {
    ptr: *mut AVIOContext,
}

impl Context {
    // TODO:
    pub fn new() -> Self {
        use std::ptr::null_mut;
        Context {
            ptr: unsafe { avio_alloc_context(null_mut(), 4096, 0, null_mut(), None, None, None) },
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut AVIOContext {
        self.ptr
    }
}

impl Context {
    pub fn open<U: AsRef<str>>(&mut self, url: U, flag: Flag) -> Result<(), Error> {
        let url = Url::try_from(url.as_ref()).map_err(|e| format!("ParseError: {:?}", e))?;
        let url = CString::new(url.as_ref()).unwrap_or_default();

        // TODO: Implement
        let int_cb = core::ptr::null_mut();
        let options: *mut *mut AVDictionary = core::ptr::null_mut();

        match unsafe { avio_open2(&mut self.ptr, url.as_ptr(), flag.into(), int_cb, options) } {
            e if e < 0 => Err(e.into()),
            _ => Ok(()),
        }
    }
}

impl From<*mut AVIOContext> for Context {
    fn from(ptr: *mut AVIOContext) -> Self {
        Context { ptr }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        match unsafe { avio_closep(&mut self.ptr) } {
            e if e < 0 => panic!("Failed to close AVIOContext: {}", e),
            _ => (),
        }
        unsafe { avio_context_free(&mut self.ptr) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use std::io::Read;
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let bytes = stream
                    .unwrap()
                    .bytes()
                    .collect::<Result<Vec<u8>, _>>()
                    .unwrap();
                println!("Received: {:?}", bytes);
            }
        });

        let mut context = Context::from(core::ptr::null_mut());
        context.open("tcp://127.0.0.1:8080", Flag::Write).unwrap();
    }
}
