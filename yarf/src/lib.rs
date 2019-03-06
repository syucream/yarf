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
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.getattr(rpath, stbuf)
}

extern "C" fn yarf_readdir_proxy(
    path: *const c_char,
    buf: *mut c_void,
    filler: FuseFillDir,
    offset: off_t,
    fi: *mut FuseFileInfo,
) -> c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.readdir(rpath, buf, filler, offset, fi)
}

extern "C" fn yarf_open_proxy(path: *const c_char, fi: *mut FuseFileInfo) -> c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.open(rpath, fi)
}

extern "C" fn yarf_read_proxy(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
    offset: off_t,
    fi: *mut FuseFileInfo,
) -> c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.read(rpath, buf, size, offset, fi)
}

// TODO more callbacks

// Convert c_char to String
fn to_rust_str(cpath: *const c_char) -> String {
    let path_cstr = unsafe { CStr::from_ptr(cpath) };
    let path_str = path_cstr.to_str().unwrap();

    return String::from(path_str);
}

// Get FileSystem trait via fuse_context
unsafe fn get_filesystem() -> Box<FileSystem> {
    let ctx = yarf_sys::fuse_get_context();
    let opsbox = (*ctx).private_data as *mut Box<FileSystem>;
    opsbox.read()
}

pub trait FileSystem {
    fn getattr(&self, _path: String, _stbuf: *mut stat) -> c_int {
        -libc::EIO
    }

    fn readdir(
        &self,
        _path: String,
        _buf: *mut c_void,
        _filler: FuseFillDir,
        _offset: off_t,
        _fi: *mut FuseFileInfo,
    ) -> c_int {
        -libc::EIO
    }

    fn open(&self, _path: String, _fi: *mut FuseFileInfo) -> c_int {
        -libc::EIO
    }

    fn read(
        &self,
        _path: String,
        _buf: *mut c_char,
        _size: usize,
        _offset: off_t,
        _fi: *mut FuseFileInfo,
    ) -> c_int {
        -libc::EIO
    }
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
        open: Some(yarf_open_proxy),
        read: Some(yarf_read_proxy),
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
        readdir: Some(yarf_readdir_proxy),
        releasedir: None,
        fsyncdir: None,
        init: None, // Unsupported because it overwrites private_data used to passing Rust value
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

    // get CLI args
    let args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    let c_args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();

    let fstmp = Box::new(fs);
    let fsptr = Box::into_raw(fstmp) as *mut Box<FileSystem> as *mut c_void;
    let opsize = mem::size_of::<FuseOperations>();

    // call fuse_main
    unsafe {
        yarf_sys::fuse_main_real(
            c_args.len() as c_int,
            c_args.as_ptr() as *mut *mut c_char,
            &ops,
            opsize,
            fsptr,
        )
    };

    0
}
