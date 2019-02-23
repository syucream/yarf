extern crate libc;
extern crate yarf;

use std::ffi::CString;
use libc::{c_char, c_int, c_uint, c_ulong, c_void, off_t, size_t, stat, uint64_t};
use yarf::{fuse_args, fuse_file_info, fuse_fill_dir_t, fuse_readdir_flags, fuse_operations};
use std::ptr::null_mut;
use std::mem::size_of_val;


extern "C" fn getattr(path: *const c_char, stbuf: *mut stat, fi: *mut fuse_file_info) -> c_int {
    0
}

extern "C" fn readdir(path: *const c_char, buf: *mut c_void, filler: fuse_fill_dir_t, offset: off_t, fi: *mut fuse_file_info, flags: fuse_readdir_flags) -> c_int {
    0
}

extern "C" fn open(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    0
}
extern "C" fn read(path: *const c_char, buf: *mut c_char, size: size_t, offset: off_t, fi: *mut fuse_file_info) -> c_int {
    0
}


fn main() {
    let ops = fuse_operations {
        getattr: getattr,
        readdir: readdir,
        open: open,
        read: read,
    };

    // args
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

    unsafe {
        let pdata: *mut c_void = null_mut();
        let opsize: size_t = size_of_val(&ops);
        yarf::fuse_main_real(c_args.len() as c_int, c_args.as_ptr(), ops, opsize, pdata)
    };
}