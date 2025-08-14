mod codec;
mod error;
mod pixel_format;
mod presets;

pub use codec::*;
use error::VideoConfigError;
use pixel_format::*;
use presets::*;

#[derive(Default)]
pub struct VideoConfig {
    pub(crate) codec: Codec,
    pub(crate) pixel_format: PixelFormat,
    pub(crate) resolution: Resolution,
    pub(crate) framerate: Framerate,
}

impl VideoConfig {
    pub fn pixel_count(&self) -> usize {
        self.resolution.pixel_count()
    }
}

impl VideoConfig {
    // codec
    /// Change the codec to AV1 and the pixel format to YUV444P.
    pub fn change_codec_to_av1(&mut self) {
        self.codec.change_to_av1();
        self.pixel_format.change_to_yuv444p();
    }
    /// Change the codec to h264 and the pixel format to YUV420P.
    pub fn change_codec_to_h264(&mut self) {
        self.codec.change_to_h264();
        self.pixel_format.change_to_yuv420p();
    }

    // resolution
    pub fn change_resolution_to_qvga(&mut self) {
        self.resolution.change_to_qvga();
    }
    pub fn change_resolution_to_vga(&mut self) {
        self.resolution.change_to_vga();
    }
    pub fn change_resolution_to_hd(&mut self) {
        self.resolution.change_to_hd();
    }
    pub fn change_resolution_to_fhd(&mut self) {
        self.resolution.change_to_fhd();
    }
    pub fn change_resolution_to_uhd4k(&mut self) {
        self.resolution.change_to_uhd4k();
    }

    // framerate
    pub fn change_framerate_to_24(&mut self) {
        self.framerate.change_to_24();
    }
    pub fn change_framerate_to_30(&mut self) {
        self.framerate.change_to_30();
    }
    pub fn change_framerate_to_60(&mut self) {
        self.framerate.change_to_60();
    }

    // pixel format
    pub fn change_pixel_format_to_yuv444p(&mut self) -> Result<(), VideoConfigError> {
        match self.codec {
            Codec::H264 => Err(VideoConfigError::UnsupportedPixelFormat),
            _ => Ok(()),
        }?;
        self.pixel_format.change_to_yuv444p();
        Ok(())
    }
    pub fn change_pixel_format_to_yuv420p(&mut self) -> Result<(), VideoConfigError> {
        match self.codec {
            _ => Ok(()),
        }?;
        self.pixel_format.change_to_yuv420p();
        Ok(())
    }
    pub fn change_pixel_format_to_rgb24(&mut self) -> Result<(), VideoConfigError> {
        match self.codec {
            Codec::Av1 | Codec::H264 => Err(VideoConfigError::UnsupportedPixelFormat),
        }?;
        self.pixel_format.change_to_rgb24();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut config = VideoConfig::default();
        config.change_codec_to_av1();
        config.change_resolution_to_hd();
        config.change_framerate_to_24();

        config.change_codec_to_h264();
        config.change_pixel_format_to_rgb24().unwrap_err();
    }
}
