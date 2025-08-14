use super::frame_data::FrameData;
use crate::codec::{Context, Packet};
use crate::util::{Frame, MediaType, PixelFormat};
use core::iter::Iterator;
use core::ops::Deref;
use kanal;
use std::rc::Rc;

pub struct PacketIterator {
    context: Context,
    packet: Rc<Packet>,
    internal_packet_chan: (
        kanal::Sender<Option<Rc<Packet>>>,
        kanal::Receiver<Option<Rc<Packet>>>,
    ),
    frame: Frame,
    frame_data_chan: (
        kanal::Sender<Option<Vec<u8>>>,
        kanal::Receiver<Option<Vec<u8>>>,
    ),
    eof: bool,
    pts: u32,
}

impl PacketIterator {
    pub fn sender(&self) -> kanal::Sender<Option<Vec<u8>>> {
        self.frame_data_chan.0.clone()
    }

    pub fn next_pts(&self) -> u32 {
        self.pts
    }

    fn frame_data(&self, data: Vec<u8>) -> FrameData {
        match self.context.codec_type() {
            MediaType::Video => match self.context.pix_fmt() {
                PixelFormat::None => panic!("PixelFormat::None is not supported"),
                PixelFormat::RGB24 => FrameData::RGB24(data),
                PixelFormat::YUV444P => FrameData::YUV444P(data),
                PixelFormat::YUV420P => unimplemented!("YUV420P is not supported yet"),
            },
            MediaType::Audio => unimplemented!("Audio is not supported yet"),
            _ => unimplemented!("Unsupported media type"),
        }
    }
}

impl Iterator for PacketIterator {
    type Item = Result<Rc<Packet>, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return match self.internal_packet_chan.1.recv() {
                Ok(Some(packet)) => Some(Ok(packet)),
                Ok(None) => None,
                Err(e) => Some(Err(e.into())),
            };
        }

        if let Err(e) = self.internal_packet_chan.0.send(None) {
            return Some(Err(e.into()));
        }

        let result = (|| {
            loop {
                if let Some(packet) = self.internal_packet_chan.1.recv()? {
                    return Ok(packet);
                };

                match self.frame_data_chan.1.recv()? {
                    None => {
                        self.frame_data_chan.1.close()?;
                        self.context.send_eof()?;
                    }
                    Some(data) => {
                        self.frame.make_writable()?;
                        self.frame_data(data)
                            .set_to_frame(&mut self.frame, &mut self.pts);
                        eprintln!("Send frame {:?}", self.frame.pts);
                        self.context.send_frame(&self.frame)?;
                    }
                };

                let mut loop_num = -1;
                loop {
                    loop_num += 1;
                    if loop_num > 0 {
                        self.packet = Rc::new(Packet::new());
                    }

                    let pkt_mut = Rc::get_mut(&mut self.packet).unwrap();
                    if let Err(e) = self.context.receive_packet(pkt_mut) {
                        match e {
                            e if e.is_eagain() || e.is_eof() => {
                                if e.is_eof() {
                                    self.eof = true;
                                }
                                self.internal_packet_chan.0.send(None)?;
                                break;
                            }
                            _ => return Err(e.into()),
                        }
                    } else {
                        eprintln!("Received packet pts: {}", self.packet.pts);
                        self.internal_packet_chan
                            .0
                            .send(Some(Rc::clone(&self.packet)))?;
                    };
                }
            }
        })();

        Some(result)
    }
}

impl Deref for PacketIterator {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}

impl Default for PacketIterator {
    fn default() -> Self {
        Self {
            context: Context::default(),
            packet: Rc::new(Packet::new()),
            internal_packet_chan: kanal::unbounded::<Option<Rc<Packet>>>(),
            frame: Frame::new(),
            frame_data_chan: kanal::bounded::<Option<Vec<u8>>>(8),
            eof: false,
            pts: 0,
        }
    }
}

impl From<Context> for PacketIterator {
    fn from(context: Context) -> Self {
        use core::cell::RefCell;
        let context = RefCell::new(context);
        let frame = Frame::try_from(&*context.borrow())
            .or_else(|e| -> Result<Frame, Box<dyn std::error::Error>> {
                eprintln!("Warning: Could not create frame from context: {}", e);
                Ok(Frame::new())
            })
            .unwrap();
        PacketIterator {
            context: context.into_inner(),
            frame,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        use crate::codec::Name;
        let pkt_iter = PacketIterator::try_from(Name::default().default_video_context()).unwrap();
        let plane_size = pkt_iter.context.width() as usize * pkt_iter.context.height() as usize;
        let sender = pkt_iter.sender();

        std::thread::spawn(move || {
            for _ in 0..100 {
                let data = vec![0u8; plane_size * 3];
                sender.send(Some(data)).unwrap();
            }
            sender.send(None).unwrap()
        });

        for result in pkt_iter {
            let _pkt = result.unwrap();
        }
    }
}
