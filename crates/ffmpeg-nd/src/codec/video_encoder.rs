//! Very experimental top-level API for video encoding.

mod config;
mod frame_data;

use super::encoder::Encoder;
use super::id::Id;
use super::packet::Packet;
use crate::util::{Frame, PixelFormat};
use config::Config;
use core::iter::FromFn;
use frame_data::FrameData;
use kanal;
use std::io::{Cursor, Write};

pub struct VideoEncoder(Encoder);

impl VideoEncoder {
    pub fn config() -> Config {
        Config::default()
    }

    fn frame_data(&self, data: Vec<u8>) -> FrameData {
        match self.0.0.pix_fmt() {
            PixelFormat::RGB24 => FrameData::RGB24(data),
            PixelFormat::YUV444P => FrameData::YUV444P(data),
            PixelFormat::YUV420P => unimplemented!("YUV420P is not supported yet"),
        }
    }

    pub fn encode(
        &mut self,
    ) -> (
        kanal::Sender<Option<Vec<u8>>>,
        FromFn<impl FnMut() -> Option<Result<Vec<u8>, Box<dyn std::error::Error>>>>,
    ) {
        let (sender, receiver) = kanal::bounded::<Option<Vec<u8>>>(3600);
        let mut frame = Frame::try_from(&self.0.0).unwrap();
        let mut packet = Packet::new();

        let mut pts = 0u32;
        let mut eof = false;

        let iter = core::iter::from_fn(move || {
            if eof {
                return None;
            }

            let mut buffer = Vec::<u8>::new();
            let mut cursor = Cursor::new(&mut buffer);

            let result: Result<(), Box<dyn std::error::Error>> = (|| {
                loop {
                    match receiver.recv()? {
                        None => {
                            receiver.close()?;
                            self.0.send_eof()?;
                        }
                        Some(data) => {
                            frame.make_writable()?;
                            self.frame_data(data).set_to_frame(&mut frame, &mut pts);
                            eprintln!("Send frame {:?}", frame.pts);
                            self.0.send_frame(&frame)?;
                        }
                    };

                    loop {
                        if let Err(e) = self.0.receive_packet(&mut packet) {
                            match e {
                                e if e.is_eagain() || e.is_eof() => {
                                    if e.is_eof() {
                                        eof = true;
                                    } else if cursor.get_ref().is_empty() {
                                        break;
                                    }
                                    return Ok(());
                                }
                                _ => return Err(e.into()),
                            }
                        } else {
                            eprintln!("Received packet with size: {}", packet.size);
                            cursor.write_all(packet.data().unwrap())?;
                            packet.unref();
                        };
                    }
                }
            })();

            Some(if let Err(e) = result {
                Err(e)
            } else {
                Ok(buffer)
            })
        });

        (sender, iter)
    }
}

impl TryFrom<&Config> for VideoEncoder {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let c = Into::<Id>::into(config.codec)
            .find_encoder()
            .unwrap()
            .into_video_context_builder()
            .framerate(config.framerate)
            .time_base(config.framerate.into_timebase())
            .width(config.resolution.width)
            .height(config.resolution.height)
            .bit_rate(400000)
            .gop_size(*config.framerate * 2)
            .pix_fmt(config.pixel_format)
            .max_b_frames(2)
            .build()?;

        Ok(VideoEncoder(c.try_into()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {
        let config = VideoEncoder::config();
        let mut video_encoder = VideoEncoder::try_from(&config).unwrap();
        let (sender, buf_iter) = video_encoder.encode();

        std::thread::spawn(move || {
            let plane_size = &config.resolution.pixel_count();
            for _ in 0..6000 {
                let data = vec![0u8; plane_size * 3];
                sender.send(Some(data)).unwrap();
            }
            sender.send(None).unwrap()
        });

        for result in buf_iter {
            match result {
                Ok(buffer) => {
                    println!("Encoded buffer size: {}", buffer.len());
                }
                Err(e) => {
                    eprintln!("Error encoding video: {:?}", e);
                    break;
                }
            }
        }
    }
}
