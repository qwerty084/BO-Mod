use std::ffi::c_void;

// addresses for BO1

// error tracking addresses
pub const COMFRAMETIME: *const c_void = 0x02481764 as *const c_void;
pub const SNAPSHOTENTITIES: *const c_void = 0x02911CB8 as *const c_void;
pub const TOTALENTITIES: *const c_void = 0x0286D034 as *const c_void;

pub const FOVPOINTER: *const c_void = 0x2FF6888 as *const c_void;
