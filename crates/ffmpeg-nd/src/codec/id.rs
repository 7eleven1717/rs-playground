use super::codec::{Codec, find_decoder, find_encoder};
use ffi::*;

pub enum Id {
    #[cfg(feature = "aom")]
    AV1,
}

impl Id {
    pub fn find_encoder(&self) -> Option<Codec> {
        find_encoder(self)
    }

    pub fn find_decoder(&self) -> Option<Codec> {
        find_decoder(self)
    }
}

impl AsRef<AVCodecID> for Id {
    fn as_ref(&self) -> &AVCodecID {
        match self {
            #[cfg(feature = "aom")]
            Id::AV1 => &AVCodecID_AV_CODEC_ID_AV1,
        }
    }
}
