use super::rational::Rational;
use ffi::*;
use std::cmp::Ordering;

pub fn compare_ts(ts_a: u32, tb_a: Rational, ts_b: u32, tb_b: Rational) -> Ordering {
    match unsafe { av_compare_ts(ts_a.into(), tb_a.into(), ts_b.into(), tb_b.into()) }
        .try_into()
        .unwrap()
    {
        0 => Ordering::Equal,
        n if n < 0 => Ordering::Less,
        _ => Ordering::Greater,
    }
}
