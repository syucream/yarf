/*
 * A Rust-friendly API for yarf-sys.
 * It supports only libfuse 2.6.x+
 *
 */
extern crate libc;
extern crate yarf_sys;

use libc::{off_t, stat};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_void};

pub type FuseConnectionInfo = ::yarf_sys::fuse_conn_info;
pub type FuseFileInfo = ::yarf_sys::fuse_file_info;
pub type FuseFillDir = ::yarf_sys::fuse_fill_dir_t;
pub type FuseOperations = ::yarf_sys::fuse_operations;

extern "C" fn yarf_getattr_proxy(path: *const c_char, stbuf: *mut stat) -> c_int {
    let ops = unsafe {
        let ctx = yarf_sys::fuse_get_context();
        let opsbox = (*ctx).private_data as *mut Box<FileSystem>;

        opsbox.as_ref().unwrap()
    };
    let rpath = to_rust_str(path);

    ops.getattr(rpath, stbuf) as c_int
}

// TODO more callbacks
/*
extern "C" fn yarf_readdir(
    path: *const c_char,
    buf: *mut c_void,
    filler: FuseFillDir,
    _offset: off_t,
    _fi: *mut FuseFillDir,
) -> c_int {
}

extern "C" fn yarf_open(path: *const c_char, _fi: *mut FuseFileInfo) -> c_int {
}

extern "C" fn yarf_read(
    path: *const c_char,
    buf: *mut c_char,
    _size: usize,
    _offset: off_t,
    _fi: *mut FuseFileInfo,
) -> c_int {
}
*/

fn to_rust_str(cpath: *const c_char) -> String {
    let path_cstr = unsafe { CStr::from_ptr(cpath) };
    let path_str = path_cstr.to_str().unwrap();

    return String::from(path_str);
}

pub trait FileSystem {
    fn getattr(&self, _path: String, _stbuf: *mut stat) -> i64 {
        -libc::EIO as i64
    }

    // TODO more methods

    /*
    fn readdir(&self, path: String, fi: FuseFileInfo) -> i64;

    fn open(&self, path: String, filler: FuseFillDir, offset: off_t, fi: *mut FuseFillDir) -> i64;

    fn read(&self, path: String, buf: *mut c_char, size: usize, offset: off_t, fi: *mut FuseFillDir) -> i64;
    */
}

pub fn yarf_main(fs: Box<FileSystem>) -> i64 {
    let ops = FuseOperations {
        // TODO more callbacks
        getattr: Some(yarf_getattr_proxy),
        readlink: None,
        getdir: None,
        mknod: None,
        mkdir: None,
        unlink: None,
        rmdir: None,
        symlink: None,
        rename: None,
        link: None,
        chmod: None,
        chown: None,
        truncate: None,
        utime: None,
        open: None,
        read: None,
        write: None,
        statfs: None,
        flush: None,
        release: None,
        fsync: None,
        setxattr: None,
        getxattr: None,
        listxattr: None,
        removexattr: None,
        opendir: None,
        readdir: None,
        releasedir: None,
        fsyncdir: None,
        init: None,
        destroy: None,
        access: None,
        create: None,
        ftruncate: None,
        fgetattr: None,
        lock: None,
        utimens: None,
        bmap: None,
        reserved00: None,
        reserved01: None,
        reserved02: None,
        reserved03: None,
        reserved04: None,
        reserved05: None,
        reserved06: None,
        reserved07: None,
        reserved08: None,
        reserved09: None,
        reserved10: None,
        setvolname: None,
        exchange: None,
        getxtimes: None,
        setbkuptime: None,
        setchgtime: None,
        setcrtime: None,
        chflags: None,
        setattr_x: None,
        fsetattr_x: None,
    };

    // args
    let args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>()
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();

    let fstmp = Box::new(fs);
    let fsptr = Box::into_raw(fstmp) as *mut Box<FileSystem> as *mut c_void;
    let opsize = mem::size_of::<FuseOperations>();

    // call fuse_main
    unsafe {
        yarf_sys::fuse_main_real(
            args.len() as c_int,
            args.as_ptr() as *mut *mut c_char,
            &ops,
            opsize,
            fsptr,
        )
    };

    0
}
