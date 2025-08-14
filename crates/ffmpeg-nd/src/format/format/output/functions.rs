use super::Output;
use ffi::*;
use std::ffi::CString;

pub fn guess_format<S: AsRef<str>>(short_name: S, filename: S, mime_type: S) -> Option<Output> {
    let s = CString::new(short_name.as_ref()).unwrap_or_default();
    let f = CString::new(filename.as_ref()).unwrap_or_default();
    let m = CString::new(mime_type.as_ref()).unwrap_or_default();
    let ptr = unsafe { av_guess_format(s.as_ptr(), f.as_ptr(), m.as_ptr()) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr.into())
    }
}

pub fn muxer_iterate() -> impl Iterator<Item = Output> {
    let mut opaque = core::ptr::null_mut();
    std::iter::from_fn(move || {
        let next = unsafe { av_muxer_iterate(&mut opaque) };
        if next.is_null() {
            None
        } else {
            Some(next.into())
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muxer_iterate() {
        for muxer in muxer_iterate() {
            println!(
                "{}: {}{}",
                muxer.name(),
                muxer.long_name(),
                muxer
                    .mime_type()
                    .map(|s| format!(" ({})", s))
                    .unwrap_or_default()
            );
        }
    }
}
