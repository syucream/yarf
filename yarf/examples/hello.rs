extern crate libc;
extern crate yarf;

use libc::stat;
use std::mem;
use std::os::raw::c_void;
use yarf::FileSystem;

const HELLO_PATH: &str = "/hello";
const HELLO_CONTENT: &str = "hello, fuse!\n";

struct HelloFS;

impl FileSystem for HelloFS {
    fn getattr(&self, path: String, stbuf: *mut stat) -> i64 {
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
                    (*stbuf).st_size = HELLO_CONTENT.len() as i64;
                }
                0
            }
            _ => -libc::ENOENT as i64,
        }
    }
}

fn main() {
    let fs = Box::new(HelloFS);

    yarf::yarf_main(fs);
}
