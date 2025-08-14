use crate::util;

#[derive(Default, Clone, Copy)]
pub enum PixelFormat {
    #[default]
    YUV444P,
    YUV420P,
    RGB24,
}

impl PixelFormat {
    pub fn change_to_yuv444p(&mut self) {
        *self = PixelFormat::YUV444P;
    }
    pub fn change_to_yuv420p(&mut self) {
        *self = PixelFormat::YUV420P;
    }
    pub fn change_to_rgb24(&mut self) {
        *self = PixelFormat::RGB24;
    }
}

impl Into<util::PixelFormat> for PixelFormat {
    fn into(self) -> util::PixelFormat {
        match self {
            PixelFormat::YUV444P => util::PixelFormat::YUV444P,
            PixelFormat::YUV420P => util::PixelFormat::YUV420P,
            PixelFormat::RGB24 => util::PixelFormat::RGB24,
        }
    }
}