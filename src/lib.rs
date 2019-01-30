extern crate libc;

use libc::{c_char, c_int, c_uint, c_ulong, c_void, off_t, size_t, uint64_t};

#[allow(non_camel_case_types)]

#[repr(C)]
pub struct fuse_args {
	argc: c_int,
	argv: *mut *mut c_char,
	allocated: c_int,
}

#[repr(C)]
pub struct fuse_opt {
	templ: *const c_char,
	offset: c_ulong,
	value: c_int,
}

#[repr(C)]
pub struct fuse_file_info {
    pub flag: c_int,
    pub feature_flags: c_uint,
    /* TODO This bit field layout is:
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

type fuse_opt_proc_t = extern fn(data: *mut c_void, arg: *const c_char, key: c_int, outargs: *mut fuse_args) -> c_int;

#[link(name = "fuse")]
extern "C" {
    pub fn fuse_opt_parse(args: *mut fuse_args, data: *mut c_void, opts: *const fuse_opt, proc: fuse_opt_proc_t) -> c_int;
    pub fn fuse_main(argc: c_int, argv: *const c_char, op: fuse_operations, private_data: *mut c_void) -> c_int;
    pub fn fuse_opt_free_args(args: *mut fuse_args) -> c_void;
}
