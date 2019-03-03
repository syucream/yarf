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
use std::cell::RefCell;

pub type FuseConnectionInfo = ::yarf_sys::fuse_conn_info;
pub type FuseFileInfo = ::yarf_sys::fuse_file_info;
pub type FuseFillDir = ::yarf_sys::fuse_fill_dir_t;
pub type FuseOperations = ::yarf_sys::fuse_operations;

thread_local!(static yarfOps: RefCell<Option<FileSystem>> = RefCell::New(None));

extern "C" fn yarf_getattr(path: *const c_char, stbuf: *mut stat) -> c_int {
    yarfOps.with(|ops_opt| {
        match ops_opt {
            Some(ops) => {
                ops.getattr() as c_int
            }
            _ => -libc::EIO
        }
    });
}

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

pub trait FileSystem {
    /*
    fn getattr(&self, path: &str, fi: FuseFileInfo) -> i64;

    fn readdir(&self, path: &str, stbuf: stat) -> i64;

    fn open(&self, path: &str, filler: FuseFillDir, offset: off_t, fi: *mut FuseFillDir) -> i64;

    fn read(&self, path: &str, buf: *mut c_char, size: usize, offset: off_t, fi: *mut FuseFillDir) -> i64;
    */
}

pub fn YarfInit(fs: &FileSystem) -> i64 {
    let ops = FuseOperations {
        getattr: Some(yarf_getattr),
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
}
