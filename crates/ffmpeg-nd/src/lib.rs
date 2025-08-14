mod codec;
mod util;

extern crate ffmpeg_nd_sys as ffi;

pub use codec::VideoEncoder;
pub use util::{available_video_encoders, version_info};
