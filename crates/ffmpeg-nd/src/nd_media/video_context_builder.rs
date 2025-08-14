//! This module is not contained within the FFmpeg API but provides a useful builder for creating codec contexts.

use crate::codec::{Codec, Context};
use crate::util::{Error, PixelFormat, Rational};

pub struct VideoContextBuilder {
    codec: Codec,
    width: Option<u16>,
    height: Option<u16>,
    framerate: Option<Rational>,
    time_base: Option<Rational>,
    bit_rate: Option<u32>,
    gop_size: Option<u8>,
    max_b_frames: Option<u8>,
    pix_fmt: Option<PixelFormat>,
}

impl VideoContextBuilder {
    pub fn new(codec: Codec) -> Self {
        VideoContextBuilder {
            codec,
            width: None,
            height: None,
            framerate: None,
            time_base: None,
            bit_rate: None,
            gop_size: None,
            max_b_frames: None,
            pix_fmt: None,
        }
    }

    pub fn width(mut self, value: u16) -> Self {
        self.width = Some(value);
        self
    }

    pub fn height(mut self, value: u16) -> Self {
        self.height = Some(value);
        self
    }

    pub fn framerate<R: Into<Rational>>(mut self, value: R) -> Self {
        self.framerate = Some(value.into());
        self
    }

    pub fn time_base<R: Into<Rational>>(mut self, value: R) -> Self {
        self.time_base = Some(value.into());
        self
    }

    pub fn bit_rate(mut self, value: u32) -> Self {
        self.bit_rate = Some(value);
        self
    }

    pub fn gop_size(mut self, value: u8) -> Self {
        self.gop_size = Some(value);
        self
    }

    pub fn max_b_frames(mut self, value: u8) -> Self {
        self.max_b_frames = Some(value);
        self
    }

    pub fn pix_fmt<P: Into<PixelFormat>>(mut self, value: P) -> Self {
        self.pix_fmt = Some(value.into());
        self
    }

    pub fn build(self) -> Result<Context, Error> {
        let mut ctx = Context::from(self.codec);
        if let Some(width) = self.width {
            ctx.set_width(width);
        }
        if let Some(height) = self.height {
            ctx.set_height(height);
        }
        if let Some(framerate) = self.framerate {
            ctx.set_frame_rate(framerate);
        }
        if let Some(time_base) = self.time_base {
            ctx.set_time_base(time_base);
        }
        if let Some(bit_rate) = self.bit_rate {
            ctx.set_bit_rate(bit_rate);
        }
        if let Some(gop_size) = self.gop_size {
            ctx.set_gop_size(gop_size);
        }
        if let Some(max_b_frames) = self.max_b_frames {
            ctx.set_max_b_frames(max_b_frames);
        }
        if let Some(pix_fmt) = self.pix_fmt {
            ctx.set_pix_fmt(pix_fmt);
        }

        ctx.open()?;
        Ok(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use crate::codec::{Name, find_encoder_by_name};
        let codec = find_encoder_by_name(&Name::default()).unwrap();

        let _context = VideoContextBuilder::new(codec)
            .pix_fmt(PixelFormat::default())
            .width(320)
            .height(240)
            .framerate(30)
            .time_base([1, 30])
            .gop_size(60)
            .max_b_frames(2)
            .build()
            .unwrap();

        let _context = Name::default().default_video_context();
    }
}
