use super::super::id::Id;
use super::Codec;
use ffi::*;

pub fn find_encoder(id: &Id) -> Option<Codec> {
    let ptr = unsafe { avcodec_find_encoder(*id.as_ref()) };
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

pub fn codec_is_encoder(codec: &Codec) -> bool {
    unsafe { av_codec_is_encoder(codec.as_ptr()) != 0 }
}
