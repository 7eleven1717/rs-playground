// https://github.com/FFmpeg/FFmpeg/blob/37507c6a78ab63bbf8dda1c0525545d30877bca2/libavutil/error.h#L57
pub const AVERROR_EOF: i32 = macros::fferrtag(b'E', b'O', b'F', b' ');

// https://github.com/FFmpeg/FFmpeg/blob/37507c6a78ab63bbf8dda1c0525545d30877bca2/doc/examples/encode_video.c#L56
pub const AVERROR_EAGAIN: i32 = macros::averror(errno::EAGAIN);

mod macros {
    // https://github.com/FFmpeg/FFmpeg/blob/37507c6a78ab63bbf8dda1c0525545d30877bca2/libavutil/error.h#L49
    pub const fn fferrtag(a: u8, b: u8, c: u8, d: u8) -> i32 {
        let tag = (a as u32) | ((b as u32) << 8) | ((c as u32) << 16) | ((d as u32) << 24);
        -(tag as i32)
    }

    // https://github.com/FFmpeg/FFmpeg/blob/37507c6a78ab63bbf8dda1c0525545d30877bca2/libavutil/error.h#L41
    pub const fn averror(e: i32) -> i32 {
        -e
    }
}

// https://github.com/torvalds/linux/blob/dfd4b508c8c6106083698a0dd5e35aecc7c48725/include/uapi/asm-generic/errno-base.h#L15
mod errno {
    pub const EAGAIN: i32 = ffi::EAGAIN;
}
