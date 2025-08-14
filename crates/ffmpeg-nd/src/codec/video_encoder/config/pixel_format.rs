use crate::util;

#[derive(Default, Clone, Copy)]
pub enum PixelFormat {
    RGB24,
    YUV420P,
    #[default]
    YUV444P,
}

impl PixelFormat {
    pub fn change_to_rgb24(&mut self) {
        *self = PixelFormat::RGB24;
    }
    pub fn change_to_yuv420p(&mut self) {
        *self = PixelFormat::YUV420P;
    }
    pub fn change_to_yuv444p(&mut self) {
        *self = PixelFormat::YUV444P;
    }
}

impl Into<util::PixelFormat> for PixelFormat {
    fn into(self) -> util::PixelFormat {
        match self {
            PixelFormat::RGB24 => util::PixelFormat::RGB24,
            PixelFormat::YUV420P => util::PixelFormat::YUV420P,
            PixelFormat::YUV444P => util::PixelFormat::YUV444P,
        }
    }
}