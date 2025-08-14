mod video;

use crate::codec::{Codec, Context, Name};
use crate::util::Error;
use crate::util::MediaType;
use core::ops::{Deref, DerefMut};
use ffi::*;
use video::*;

pub struct Encoder(pub Context);

impl Encoder {
    pub fn video_config() -> VideoConfig {
        VideoConfig::default()
    }
}

impl Deref for Encoder {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<&VideoConfig> for Encoder {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: &VideoConfig) -> Result<Self, Self::Error> {
        let c = Into::<Name>::into(config.codec)
            .try_into_video_encode_context_builder()
            .unwrap()
            .framerate(config.framerate)
            .time_base(config.framerate.into_timebase())
            .width(config.resolution.width)
            .height(config.resolution.height)
            // .bit_rate(400000)
            .gop_size(*config.framerate * 2)
            .pix_fmt(config.pixel_format)
            .max_b_frames(2)
            .build()?;

        Ok(c.try_into()?)
    }
}

impl TryFrom<Context> for Encoder {
    type Error = Error;

    fn try_from(context: Context) -> Result<Self, Self::Error> {
        match unsafe { av_codec_is_encoder(context.codec) } {
            e if e == 0 => Err(e.into()),
            _ => match context.codec_type() {
                MediaType::Video | MediaType::Audio => Ok(Self(context)),
                _ => unimplemented!("Unsupported media type"),
            },
        }
    }
}

impl TryFrom<Codec> for Encoder {
    type Error = Error;

    fn try_from(codec: Codec) -> Result<Self, Self::Error> {
        codec.try_into()
    }
}

use super::packet_iterator::PacketIterator;
use crate::codec::Packet;

use std::rc::Rc;
impl IntoIterator for Encoder {
    type Item = Result<Rc<Packet>, Box<dyn std::error::Error>>;
    type IntoIter = PacketIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use crate::codec::Name;
        let encoder: Encoder = Name::default().default_video_context().try_into().unwrap();
        let _pkt_iter = encoder.into_iter();
    }
}
