extern crate libc;
extern crate yarf;

use libc::{size_t, stat};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_ulonglong, c_void};
use std::ptr;
use std::ptr::null_mut;
use yarf::{fuse_conn_info, fuse_file_info, fuse_fill_dir_t, fuse_operations, off_t};

const HELLO_PATH: &str = "/hello";
const HELLO_CONTENT: &str = "hello, fuse!\n";

extern "C" fn yarf_init(_conn: *mut fuse_conn_info) -> *mut ::std::os::raw::c_void {
    null_mut() as *mut c_void
}

extern "C" fn yarf_getattr(path: *const c_char, stbuf: *mut stat) -> c_int {
    let path_str = unsafe { CStr::from_ptr(path) };
    let path_slice = path_str.to_str().unwrap();

    // zero fill
    unsafe {
        libc::memset(stbuf as *mut c_void, 0, mem::size_of_val(&stbuf));
    }

    match path_slice {
        "/" => {
            unsafe {
                (*stbuf).st_mode = libc::S_IFDIR | 0o755;
                (*stbuf).st_nlink = 2;
            }
            0
        }
        HELLO_PATH => {
            unsafe {
                (*stbuf).st_mode = libc::S_IFREG | 0o444;
                (*stbuf).st_nlink = 1;
                (*stbuf).st_size = HELLO_CONTENT.len() as i64;
            }
            0
        }
        _ => -libc::ENOENT
    }
}

extern "C" fn yarf_readdir(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse_fill_dir_t,
    _offset: off_t,
    _fi: *mut fuse_file_info,
) -> c_int {
    let path_str = unsafe { CStr::from_ptr(path) };
    let path_slice = path_str.to_str().unwrap();

    match path_slice {
        "/" => {
            match filler {
                Some(filler_func) => unsafe {
                    let current_dir = CString::new(".").unwrap();
                    let parent_dir = CString::new("..").unwrap();
                    let hello_file = CString::new("hello").unwrap();

                    filler_func(
                        buf,
                        current_dir.as_ptr(),
                        ptr::null_mut(),
                        0
                    );
                    filler_func(
                        buf,
                        parent_dir.as_ptr(),
                        ptr::null_mut(),
                        0
                    );
                    filler_func(
                        buf,
                        hello_file.as_ptr(),
                        ptr::null_mut(),
                        0
                    );
                }
                _ => {}
            }
            0
        }

        _ => -libc::ENOENT,
    }
}

extern "C" fn yarf_open(path: *const c_char, _fi: *mut fuse_file_info) -> c_int {
    let path_str = unsafe { CStr::from_ptr(path) };
    let path_slice = path_str.to_str().unwrap();

    match path_slice {
        HELLO_PATH => 0,
        _ => -libc::ENOENT,
    }
}

// FIXME something wrong
extern "C" fn yarf_read(
    path: *const c_char,
    buf: *mut c_char,
    _size: usize,
    _offset: off_t,
    _fi: *mut fuse_file_info,
) -> c_int {
    let path_str = unsafe { CStr::from_ptr(path) };
    let path_slice = path_str.to_str().unwrap();

    match path_slice {
        HELLO_PATH => {
            let content = CString::new(HELLO_CONTENT).unwrap();
            let content_len = HELLO_CONTENT.len();
            unsafe {
                ptr::copy_nonoverlapping(content.as_ptr(), buf, content_len);
            }
            content_len as c_int
        }
        _ => -libc::ENOENT,
    }
}

fn main() {
    let ops = fuse_operations {
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
        open: Some(yarf_open),
        read: Some(yarf_read),
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
        readdir: Some(yarf_readdir),
        releasedir: None,
        fsyncdir: None,
        init: Some(yarf_init),
        destroy: None,
        access: None,
        create: None,
        ftruncate: None,
        fgetattr: None,
        lock: None,
        utimens: None,
        bmap: None
    };

    // args
    let args = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect::<Vec<CString>>();
    let c_args = args
        .iter()
        .map(|arg| arg.as_ptr())
        .collect::<Vec<*const c_char>>();

    let pdata: *mut c_void = ptr::null_mut();
    // TODO use side_of
    let opsize: size_t = mem::size_of_val(&ops);
    unsafe {
        yarf::fuse_main_real(
            c_args.len() as c_int,
            c_args.as_ptr() as *mut *mut c_char,
            &ops,
            opsize,
            pdata,
        )
    };
}
