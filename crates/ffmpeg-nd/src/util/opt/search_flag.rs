use core::ffi::c_int;
use ffi::*;

pub enum SearchFlag {
    Children,
    FakeObj,
}

impl From<c_int> for SearchFlag {
    fn from(value: c_int) -> Self {
        match value {
            AV_OPT_SEARCH_CHILDREN => SearchFlag::Children,
            AV_OPT_SEARCH_FAKE_OBJ => SearchFlag::FakeObj,
            _ => panic!("Invalid search flag"),
        }
    }
}

impl Into<c_int> for SearchFlag {
    fn into(self) -> c_int {
        match self {
            SearchFlag::Children => AV_OPT_SEARCH_CHILDREN,
            SearchFlag::FakeObj => AV_OPT_SEARCH_FAKE_OBJ,
        }
    }
}
