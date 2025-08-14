use crate::util::Frame;
use core::slice;

pub enum FrameData {
    /// Packed format.
    RGB24(Vec<u8>),
    /// Planar format.
    YUV420P(Vec<u8>),
    /// Planar format.
    YUV444P(Vec<u8>),
}

impl FrameData {
    #[inline(always)]
    pub fn set_to_frame(&self, frame: &mut Frame, pts: &mut u32) {
        match self {
            FrameData::RGB24(data) => {
                // RGBRGB...
                let len = frame.pixel_count() * 3;
                let dst = unsafe { slice::from_raw_parts_mut(frame.data[0], len) };
                dst.copy_from_slice(data);
            }
            FrameData::YUV420P(data) => {
                // YUV420P
                let plane_size = frame.pixel_count();
                let (y_plane, rest) = data.split_at(plane_size);
                let (u_plane, v_plane) = rest.split_at(plane_size / 4);
                let y_dst = unsafe { slice::from_raw_parts_mut(frame.data[0], plane_size) };
                let u_dst = unsafe { slice::from_raw_parts_mut(frame.data[1], plane_size / 4) };
                let v_dst = unsafe { slice::from_raw_parts_mut(frame.data[2], plane_size / 4) };
                y_dst.copy_from_slice(y_plane);
                u_dst.copy_from_slice(u_plane);
                v_dst.copy_from_slice(v_plane);
            }
            FrameData::YUV444P(data) => {
                let plane_size = frame.pixel_count();

                let (y_plane, rest) = data.split_at(plane_size);
                let (u_plane, v_plane) = rest.split_at(plane_size);

                let y_dst = unsafe { slice::from_raw_parts_mut(frame.data[0], plane_size) };
                let u_dst = unsafe { slice::from_raw_parts_mut(frame.data[1], plane_size) };
                let v_dst = unsafe { slice::from_raw_parts_mut(frame.data[2], plane_size) };

                y_dst.copy_from_slice(y_plane);
                u_dst.copy_from_slice(u_plane);
                v_dst.copy_from_slice(v_plane);
            }
        }

        frame.set_pts(*pts);
        *pts += 1;
    }
}
