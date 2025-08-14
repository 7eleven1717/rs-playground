use crate::codec::{Codec, codec_iterate};
use crate::util::MediaType;

pub fn video_encoder_iterate() -> impl Iterator<Item = Codec> {
    codec_iterate().filter(|codec| codec.is_encoder() && codec.type_() == MediaType::Video)
}

pub fn video_decoder_iterate() -> impl Iterator<Item = Codec> {
    codec_iterate().filter(|codec| codec.is_decoder() && codec.type_() == MediaType::Video)
}

pub fn audio_encoder_iterate() -> impl Iterator<Item = Codec> {
    codec_iterate().filter(|codec| codec.is_encoder() && codec.type_() == MediaType::Audio)
}

pub fn audio_decoder_iterate() -> impl Iterator<Item = Codec> {
    codec_iterate().filter(|codec| codec.is_decoder() && codec.type_() == MediaType::Audio)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_encoder_iterate() {
        for codec in video_encoder_iterate() {
            println!("{}: {}", codec.name(), codec.long_name());
        }
    }

    #[test]
    fn test_video_decoder_iterate() {
        for codec in video_decoder_iterate() {
            println!("{}: {}", codec.name(), codec.long_name());
        }
    }
}
