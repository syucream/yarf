extern crate libc;
extern crate yarf;

use libc::{off_t, stat};
use std::io::Write;
use std::os::raw::c_int;
use std::ptr;
use yarf::ReadDirFiller;
use yarf::{FileSystem, FuseFileInfo};

const HELLO_PATH: &str = "/hello";
const HELLO_CONTENT: &str = "Hello, World!\n";

struct HelloFS;

impl FileSystem for HelloFS {
    fn getattr(&self, path: String, stbuf: Option<&mut stat>) -> c_int {
        match path.as_str() {
            "/" => {
                let mut st = stbuf.unwrap();
                st.st_mode = libc::S_IFDIR | 0o755;
                st.st_nlink = 2;
                0
            }
            HELLO_PATH => {
                let mut st = stbuf.unwrap();
                st.st_mode = libc::S_IFREG | 0o444;
                st.st_nlink = 1;
                st.st_size = (HELLO_CONTENT.len() as c_int).into();
                0
            }
            _ => -libc::ENOENT,
        }
    }

    fn open(&self, path: String, _fi: Option<&mut FuseFileInfo>) -> c_int {
        match path.as_str() {
            HELLO_PATH => 0,
            _ => -libc::ENOENT,
        }
    }

    fn read(
        &self,
        path: String,
        buf: &mut [u8],
        _offset: off_t,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        match path.as_str() {
            HELLO_PATH => {
                let content_len = HELLO_CONTENT.len();
                let mut wbuf = buf;
                wbuf.write(HELLO_CONTENT.as_bytes()).unwrap();
                content_len as c_int
            }
            _ => -libc::ENOENT,
        }
    }

    fn readdir(
        &self,
        path: String,
        filler: ReadDirFiller,
        _offset: off_t,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        match path.as_str() {
            "/" => {
                filler.fill(".", ptr::null(), 0);
                filler.fill("..", ptr::null(), 0);
                filler.fill("hello", ptr::null(), 0);
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
