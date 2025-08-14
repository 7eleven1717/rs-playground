#![allow(non_upper_case_globals)]
use ffi::*;

pub enum PixelFormat {
    RGB24,
    YUV420P,
    YUV444P,
}

impl From<AVPixelFormat> for PixelFormat {
    fn from(value: AVPixelFormat) -> Self {
        match value {
            AVPixelFormat_AV_PIX_FMT_RGB24 => PixelFormat::RGB24,
            AVPixelFormat_AV_PIX_FMT_YUV420P => PixelFormat::YUV420P,
            AVPixelFormat_AV_PIX_FMT_YUV444P => PixelFormat::YUV444P,
            _ => panic!("Unsupported pixel format"),
        }
    }
}

impl Into<AVPixelFormat> for PixelFormat {
    fn into(self) -> AVPixelFormat {
        match self {
            PixelFormat::RGB24 => AVPixelFormat_AV_PIX_FMT_RGB24,
            PixelFormat::YUV420P => AVPixelFormat_AV_PIX_FMT_YUV420P,
            PixelFormat::YUV444P => AVPixelFormat_AV_PIX_FMT_YUV444P,
        }
    }
}
