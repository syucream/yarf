extern crate libc;

use libc::{c_int, c_char, c_uchar, c_uint, off_t, size_t, uint64_t};

#[allow(non_camel_case_types)]

#[repr(C)]
pub struct fuse_file_info {
    pub flag: c_int,
    pub feature_flags: c_uint,
    /* This bit field layout is:
        pub writepage: c_uint,
        pub direct_io: c_uint,
        pub keep_cache: c_uint,
        pub flush: c_uint,
        pub nonseekable: c_uint,
        pub flock_release: c_uint,
        pub padding: c_uint,
    */
    pub fh: uint64_t,
    pub lock_owner: uint64_t,
    pub poll_events: uint64_t,
}

#[repr(C)]
pub struct fuse_operations {
    // pub getattr: fn(self: *mut fuse_operations, path: *const c_char) -> c_int,
    pub read: extern fn(path: *const c_char, buf: *mut c_char, size: size_t, offset: off_t, fi: *mut fuse_file_info) -> c_int,
}
