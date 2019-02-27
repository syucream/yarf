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
const HELLO_CONTENT: &str = "hello, fuse!";

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
                (*stbuf).st_mode = libc::S_IFDIR | 0755;
                (*stbuf).st_nlink = 2;
            }
            0
        }
        HELLO_PATH => {
            unsafe {
                (*stbuf).st_mode = libc::S_IFREG | 0444;
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
                    filler_func(
                        buf,
                        CString::new(".").unwrap().as_ptr(),
                        ptr::null_mut(),
                        0
                    );
                    filler_func(
                        buf,
                        CString::new("..").unwrap().as_ptr(),
                        ptr::null_mut(),
                        0
                    );
                    filler_func(
                        buf,
                        CString::new("hello").unwrap().as_ptr(),
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
            unsafe {
                let content_ptr = CString::new(HELLO_CONTENT).unwrap().as_ptr();
                let content_len = HELLO_CONTENT.len();
                ptr::copy_nonoverlapping(content_ptr, buf, content_len);
            }
            0
        }
        _ => -libc::ENOENT,
    }
}

fn main() {
    let ops = fuse_operations {
        init: Some(yarf_init),
        getattr: Some(yarf_getattr),
        readdir: Some(yarf_readdir),
        open: Some(yarf_open),
        read: Some(yarf_read),
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
