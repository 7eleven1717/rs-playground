use super::codec::Codec;
use super::context::Context;
use super::packet::Packet;
use crate::util::{Error, Frame};
use core::ops::{Deref, DerefMut};
use ffi::*;

pub struct Encoder(pub Context);

impl Encoder {
    pub fn send_frame(&mut self, frame: &Frame) -> Result<(), Error> {
        match unsafe { avcodec_send_frame(self.as_mut_ptr(), frame.as_ptr()) } {
            e if e < 0 => Err(Error::from(e)),
            _ => Ok(()),
        }
    }

    pub fn send_eof(&mut self) -> Result<(), Error> {
        self.send_frame(&Frame::null())
    }

    pub fn receive_packet(&mut self, packet: &mut Packet) -> Result<(), Error> {
        match unsafe { avcodec_receive_packet(self.as_mut_ptr(), packet.as_mut_ptr()) } {
            e if e < 0 => Err(Error::from(e)),
            _ => Ok(()),
        }
    }
}

impl TryFrom<Context> for Encoder {
    type Error = Error;

    fn try_from(context: Context) -> Result<Self, Self::Error> {
        match unsafe { av_codec_is_encoder(context.codec) } {
            e if e == 0 => Err(Error::from(e)),
            _ => Ok(Encoder(context)),
        }
    }
}

impl TryFrom<Codec> for Encoder {
    type Error = Error;

    fn try_from(codec: Codec) -> Result<Self, Self::Error> {
        codec.try_into()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use crate::codec::id::Id;
        let _encoder: Encoder = Id::AV1
            .find_encoder()
            .unwrap()
            .into_video_context_builder()
            .build()
            .unwrap()
            .try_into()
            .unwrap();
    }
}
