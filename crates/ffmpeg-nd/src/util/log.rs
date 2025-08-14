mod level;

pub use level::Level;
use ffi::*;

pub fn set_level(level: Level) {
    unsafe {
        av_log_set_level(level.into());
    }
}
