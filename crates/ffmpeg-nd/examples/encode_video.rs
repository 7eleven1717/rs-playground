use ffmpeg_nd::VideoEncoder;

pub fn main() {
    let config = VideoEncoder::config();
    let mut video_encoder = VideoEncoder::try_from(&config).unwrap();
    let (sender, buf_iter) = video_encoder.encode();
}
