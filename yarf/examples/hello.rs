extern crate libc;
extern crate yarf;

use libc::{off_t, stat};
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use yarf::{FileSystem, FuseFileInfo, FuseFillDir};

const HELLO_PATH: &str = "/hello";
const HELLO_CONTENT: &str = "hello, fuse!\n";

struct HelloFS;

impl FileSystem for HelloFS {
    fn getattr(&self, path: String, stbuf: *mut stat) -> c_int {
        // zero fill
        unsafe {
            libc::memset(stbuf as *mut c_void, 0, mem::size_of_val(&stbuf));
        }

        match path.as_str() {
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
                    (*stbuf).st_size = (HELLO_CONTENT.len() as c_int).into()
                }
                0
            }
            _ => -libc::ENOENT,
        }
    }

    fn open(&self, path: String, _fi: *mut FuseFileInfo) -> c_int {
        match path.as_str() {
            HELLO_PATH => 0,
            _ => -libc::ENOENT,
        }
    }

    fn read(
        &self,
        path: String,
        buf: *mut c_char,
        _size: usize,
        _offset: off_t,
        _fi: *mut FuseFileInfo,
    ) -> c_int {
        match path.as_str() {
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

    fn readdir(
        &self,
        path: String,
        buf: *mut c_void,
        filler: FuseFillDir,
        _offset: off_t,
        _fi: *mut FuseFileInfo,
    ) -> c_int {
        match path.as_str() {
            "/" => {
                match filler {
                    Some(filler_func) => unsafe {
                        let current_dir = CString::new(".").unwrap();
                        let parent_dir = CString::new("..").unwrap();
                        let hello_file = CString::new("hello").unwrap();

                        filler_func(buf, current_dir.as_ptr(), ptr::null_mut(), 0);
                        filler_func(buf, parent_dir.as_ptr(), ptr::null_mut(), 0);
                        filler_func(buf, hello_file.as_ptr(), ptr::null_mut(), 0);
                    },
                    _ => {}
                }
                0
            }
            _ => -libc::ENOENT,
        }
    }
}

fn main() {
    let fs = Box::new(HelloFS);

    yarf::yarf_main(fs);
}
