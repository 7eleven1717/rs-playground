use super::super::id::Id;
use super::Codec;
use ffi::*;
use std::ffi::CString;

pub fn find_encoder(id: &Id) -> Option<Codec> {
    let ptr = unsafe { avcodec_find_encoder(*id.as_ref()) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr.into())
    }
}

pub fn find_encoder_by_name<T: AsRef<str>>(name: T) -> Option<Codec> {
    let name = CString::new(name.as_ref()).unwrap_or_default();
    let ptr = unsafe { avcodec_find_encoder_by_name(name.as_ptr()) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr.into())
    }
}

pub fn find_decoder(id: &Id) -> Option<Codec> {
    let ptr = unsafe { avcodec_find_decoder(*id.as_ref()) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr.into())
    }
}

pub fn find_decoder_by_name<T: AsRef<str>>(name: T) -> Option<Codec> {
    let name = CString::new(name.as_ref()).unwrap_or_default();
    let ptr = unsafe { avcodec_find_decoder_by_name(name.as_ptr()) };
    if ptr.is_null() {
        None
    } else {
        Some(ptr.into())
    }
}

pub fn codec_iterate() -> impl Iterator<Item = Codec> {
    let mut opaque = std::ptr::null_mut();
    core::iter::from_fn(move || {
        let ptr = unsafe { av_codec_iterate(&mut opaque) };
        if ptr.is_null() {
            return None;
        }
        Some(ptr.into())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_iterate() {
        for codec in codec_iterate() {
            let codec_type = if codec.is_encoder() { "[encoder]" } else { "[decoder]" };
            println!("{} (name: {}) (long_name: {})", codec_type, codec.name(), codec.long_name());
        }
    }
}
