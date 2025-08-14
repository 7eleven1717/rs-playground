use super::packet_iterator::PacketIterator;
use crate::format::{Context, Stream};
use crate::util::{Error, compare_ts};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct Muxer {
    context: Context,
    streams: Vec<(Stream, PacketIterator)>,
}

impl Muxer {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            streams: Vec::new(),
        }
    }

    // TODO: Consider adding this method to the struct
    #[cfg(test)]
    pub fn add_stream_from_pkt_iter(&mut self, pkt_iter: PacketIterator) {
        let stream: Stream = (&mut self.context, &*pkt_iter).try_into().unwrap();
        self.streams.push((stream, pkt_iter));
    }

    fn next_stream(&mut self) -> (usize, &mut (Stream, PacketIterator)) {
        self.streams
            .iter_mut()
            .enumerate()
            .min_by(|(_, (_, a)), (_, (_, b))| {
                compare_ts(a.next_pts(), a.time_base(), b.next_pts(), b.time_base())
            })
            .unwrap()
    }

    pub fn mux(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.context.write_header()?;
        loop {
            let (index, (stream, pkt_iter)) = self.next_stream();
            match pkt_iter.next() {
                Some(Ok(mut pkt)) => {
                    let pkt = Rc::get_mut(&mut pkt).unwrap();
                    pkt.rescale_ts(&pkt_iter.time_base(), &stream.time_base());
                    pkt.set_stream_index(stream.index());
                    self.context.interleaved_write_frame(pkt)?;
                }
                Some(Err(e)) => return Err(e),
                None => {
                    self.streams.remove(index);
                    if self.streams.len() == 0 {
                        break;
                    }
                }
            }
        }
        self.context.write_trailer()?;
        Ok(())
    }
}

type CtxWithPktIters = (Context, Vec<PacketIterator>);

impl TryFrom<CtxWithPktIters> for Muxer {
    type Error = Error;

    fn try_from((context, pkt_iters): CtxWithPktIters) -> Result<Self, Self::Error> {
        let context = RefCell::new(context);
        let streams = pkt_iters
            .into_iter()
            .map(|pkt_iter| {
                let stream = Stream::try_from((&mut *context.borrow_mut(), &*pkt_iter))?;
                Ok((stream, pkt_iter))
            })
            .collect::<Result<Vec<(Stream, PacketIterator)>, Self::Error>>()?;

        Ok(Muxer {
            context: context.into_inner(),
            streams,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_mux() {
        use crate::{Encoder, LogLevel};
        use tempfile::NamedTempFile;

        LogLevel::Trace.set();

        let video_config = Encoder::video_config();
        let video_pkt_iter: PacketIterator = Encoder::try_from(&video_config).unwrap().into_iter();
        let video_frame_data_sender = video_pkt_iter.sender();
        let plane_size = video_pkt_iter.width() as usize * video_pkt_iter.height() as usize;

        std::thread::spawn(move || {
            for _ in 0..120 {
                let data = vec![0u8; plane_size * 3];
                video_frame_data_sender.send(Some(data)).unwrap();
            }
            video_frame_data_sender.send(None).unwrap()
        });

        let mut named_temp_file = NamedTempFile::new().unwrap();
        let url = format!("file://{}", named_temp_file.path().display());

        let mut context = Context::default();
        context.open_write(&url).unwrap();

        let mut muxer = Muxer::try_from((context, vec![video_pkt_iter])).unwrap();
        muxer.mux().unwrap();

        // To keep the temporary file for inspection (optional)
        use std::io::Write;
        named_temp_file.disable_cleanup(false);
        writeln!(std::io::stdout(), "Output file: {}", &url).unwrap();
    }
}
