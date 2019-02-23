extern crate libc;

use libc::{c_char, c_int, c_uint, c_ulong, c_void, off_t, size_t, stat, uint64_t};

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct fuse_args {
	argc: c_int,
	argv: *mut *mut c_char,
	allocated: c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct fuse_opt {
	templ: *const c_char,
	offset: c_ulong,
	value: c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
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
#[allow(non_camel_case_types)]
pub enum fuse_readdir_flags {
    FUSE_READDIR_PLUS = (1 << 1)
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum fuse_fill_dir_flags {
    FUSE_FILL_DIR_PLUS = (1 << 1)
}

#[allow(non_camel_case_types)]
type fuse_fill_dir_t = extern fn(
    buf: *mut c_void, path: *const c_char, stbuf: *const stat, off: off_t, flags: fuse_fill_dir_flags
) -> c_int;

#[allow(non_camel_case_types)]
type fuse_opt_proc_t = extern fn(data: *mut c_void, arg: *const c_char, key: c_int, outargs: *mut fuse_args) -> c_int;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct fuse_operations {
    pub getattr: extern fn(path: *const c_char, stbuf: *mut stat, fi: *mut fuse_file_info) -> c_int,
    pub readdir: extern fn(path: *const c_char, buf: *mut c_void, filler: fuse_fill_dir_t, offset: off_t, fi: *mut fuse_file_info, flags: fuse_readdir_flags) -> c_int,
    pub open: extern fn(path: *const c_char, fi: *mut fuse_file_info) -> c_int,
    pub read: extern fn(path: *const c_char, buf: *mut c_char, size: size_t, offset: off_t, fi: *mut fuse_file_info) -> c_int,
}

#[link(name = "fuse")]
extern "C" {
    pub fn fuse_opt_parse(args: *mut fuse_args, data: *mut c_void, opts: *const fuse_opt, proc: fuse_opt_proc_t) -> c_int;
    pub fn fuse_main(argc: c_int, argv: *const *const c_char, op: fuse_operations, private_data: *mut c_void) -> c_int;
    pub fn fuse_opt_free_args(args: *mut fuse_args) -> c_void;
}
