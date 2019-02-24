extern crate libc;
extern crate yarf;

use libc::ENOENT;
use libc::{off_t, size_t, stat};
use std::ptr;
use std::ptr::null_mut;
use std::mem::size_of_val;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void, c_ulonglong};
use std::ffi::CStr;
use std::ffi::CString;
use yarf::{fuse_args, fuse_file_info, fuse_fill_dir_t, fuse_readdir_flags, fuse_operations};

const helloPath: &str = "/hello";
const helloContent: &str = "hello, fuse!";

extern "C" fn getattr(path: *const c_char, stbuf: *mut stat, fi: *mut fuse_file_info) -> c_int {
    let pathStr = unsafe { CStr::from_ptr(path) };
    let pathSlice = pathStr.to_str().unwrap();

    match pathSlice {
        "/" => {
            unsafe {
            }
            0
        }
        helloPath => {
            0
        }
        _ => {
            ENOENT
        }
    }
}

extern "C" fn readdir(path: *const c_char, buf: *mut c_void, filler: fuse_fill_dir_t, offset: off_t, fi: *mut fuse_file_info, flags: fuse_readdir_flags) -> c_int {
    let pathStr = unsafe { CStr::from_ptr(path) };
    let pathSlice = pathStr.to_str().unwrap();

    match pathSlice {
        "/" => {
            ENOENT
        }
        _ => {
            unsafe {
                filler(buf, CString::new(".").unwrap().as_ptr(), null_mut(), 0, yarf::fuse_fill_dir_flags::FUSE_FILL_DIR_ZERO);
                filler(buf, CString::new("..").unwrap().as_ptr(), null_mut(), 0, yarf::fuse_fill_dir_flags::FUSE_FILL_DIR_ZERO);
                filler(buf, CString::new("hello").unwrap().as_ptr(), null_mut(), 0, yarf::fuse_fill_dir_flags::FUSE_FILL_DIR_ZERO);
            }
            0
        }
    }
}

extern "C" fn open(path: *const c_char, fi: *mut fuse_file_info) -> c_int {
    let pathStr = unsafe { CStr::from_ptr(path) };
    let pathSlice = pathStr.to_str().unwrap();

    match pathSlice {
        helloPath => {
            0
        }
        _ => {
            ENOENT
        }
    }
}
extern "C" fn read(path: *const c_char, buf: *mut c_char, size: size_t, offset: off_t, fi: *mut fuse_file_info) -> c_int {
    let pathStr = unsafe { CStr::from_ptr(path) };
    let pathSlice = pathStr.to_str().unwrap();

    match pathSlice {
        helloPath => {
            unsafe {
                let helloContentPtr = unsafe { CString::new(helloContent).unwrap().as_ptr() };
                let helloContentLen = helloContent.len();
                ptr::copy_nonoverlapping(helloContentPtr, buf, helloContentLen);
            }
            0
        }
        _ => {
            ENOENT
        }
    }
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