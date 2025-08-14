use super::super::id::Id;
use crate::codec::find_encoder_by_name;
use crate::nd_media::VideoContextBuilder;
use crate::util::Error;

pub enum Name {
    #[cfg(feature = "aom")]
    LibAomAv1,
    #[cfg(feature = "x264")]
    LibX264,
    #[cfg(feature = "openh264")]
    LibOpenH264,
}

impl Name {
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "aom")]
            Name::LibAomAv1 => "libaom-av1",
            #[cfg(feature = "x264")]
            Name::LibX264 => "libx264",
            #[cfg(feature = "openh264")]
            Name::LibOpenH264 => "libopenh264",
        }
    }

    pub fn try_into_video_encode_context_builder(self) -> Result<VideoContextBuilder, Error> {
        if let Some(encoder) = find_encoder_by_name(&self) {
            Ok(VideoContextBuilder::new(encoder))
        } else {
            Err(format!("Could not find encoder for '{}'", self.name()).into())
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl From<Name> for Id {
    fn from(name: Name) -> Self {
        match name {
            #[cfg(feature = "aom")]
            Name::LibAomAv1 => Id::AV1,
            #[cfg(feature = "x264")]
            Name::LibX264 => Id::H264,
            #[cfg(feature = "openh264")]
            Name::LibOpenH264 => Id::H264,
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::super::context::Context;
    use super::*;
    use crate::util::PixelFormat;

    impl Default for Name {
        fn default() -> Self {
            Name::LibAomAv1
        }
    }

    impl Name {
        /// Experimental and testing purpose only now.
        pub fn default_video_context(self) -> Context {
            self.try_into_video_encode_context_builder()
                .unwrap()
                .pix_fmt(PixelFormat::default())
                .width(320)
                .height(240)
                .framerate(30)
                .time_base([1, 30])
                .gop_size(60)
                .max_b_frames(2)
                .build()
                .unwrap()
        }
    }
}
