mod codec;
mod pixel_format;
mod presets;
pub use codec::*;
use pixel_format::*;
use presets::*;

#[derive(Default)]
pub struct Config {
    pub(super) codec: Codec,
    pub(super) pixel_format: PixelFormat,
    pub(super) resolution: Resolution,
    pub(super) framerate: Framerate,
}

impl Config {
    // codec
    pub fn change_codec_to_av1(&mut self) {
        self.codec.change_to_av1();
    }
    pub fn change_codec_to_h264(&mut self) {
        self.codec.change_to_h264();
    }

    // pixel format
    pub fn change_pixel_format_to_rgb24(&mut self) {
        self.pixel_format.change_to_rgb24();
    }
    pub fn change_pixel_format_to_yuv444p(&mut self) {
        self.pixel_format.change_to_yuv444p();
    }
    pub fn change_pixel_format_to_yuv420p(&mut self) {
        self.pixel_format.change_to_yuv420p();
    }

    // resolution
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn playground() {
        let mut config = Config::default();
        config.change_codec_to_av1();
        config.change_pixel_format_to_rgb24();
        config.change_resolution_to_hd();
        config.change_framerate_to_24();
    }
}
