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
use std::slice;

pub type FuseConnectionInfo = ::yarf_sys::fuse_conn_info;
pub type FuseDirHandler = ::yarf_sys::fuse_dirh_t;
pub type FuseDirFil = ::yarf_sys::fuse_dirfil_t;
pub type FuseFileInfo = ::yarf_sys::fuse_file_info;
pub type FuseOperations = ::yarf_sys::fuse_operations;

#[doc = "A filesystem on FUSE"]
pub trait FileSystem {
    #[doc = " Get file attributes."]
    fn getattr(&self, _path: String, _stbuf: Option<&mut stat>) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Read the target of a symbolic link"]
    fn readlink(&self, _path: String, _buf: &mut [u8]) -> ::std::os::raw::c_int {
        -libc::ENOSYS
    }

    fn getdir(&self, _path: String, _arg2: FuseDirHandler, _arg3: FuseDirFil) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Create a file node"]
    fn mknod(&self, _path: String, _mode: ::libc::mode_t, _dev: ::libc::dev_t) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Create a directory"]
    fn mkdir(&self, _path: String, _mode: ::libc::mode_t) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Remove a file"]
    fn unlink(&self, _path: String) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Remove a directory"]
    fn rmdir(&self, _path: String) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Create a symbolic link"]
    fn symlink(&self, _path1: String, _path2: String) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Rename a file"]
    fn rename(&self, _old: String, _new: String) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Create a hard link to a file"]
    fn link(&self, _path1: String, _path2: String) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Change the permission bits of a file"]
    fn chmod(&self, _path: String, _mode: ::libc::mode_t) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Change the owner and group of a file"]
    fn chown(&self, _path: String, _uid: ::libc::uid_t, _gid: ::libc::gid_t) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Change the size of a file"]
    fn truncate(&self, _path: String, _offset: ::libc::off_t) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Change the access and/or modification times of a file"]
    fn utime(&self, _path: String, _utimbuf: Option<&mut ::libc::utimbuf>) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " File open operation"]
    fn open(&self, _path: String, _fi: Option<&mut FuseFileInfo>) -> c_int {
        0
    }

    #[doc = " Read data from an open file"]
    fn read(
        &self,
        _path: String,
        _buf: &mut [u8],
        _offset: off_t,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Write data to an open file"]
    fn write(
        &self,
        _path: String,
        _buf: &[u8],
        _offset: ::libc::off_t,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Get file system statistics"]
    fn statfs(&self, _path: String, _statvfs: Option<&mut ::libc::statvfs>) -> c_int {
        0
    }

    #[doc = " Possibly flush cached data"]
    fn flush(&self, _path: String, _fi: Option<&mut FuseFileInfo>) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Release an open file"]
    fn release(&self, _path: String, _fi: Option<&mut FuseFileInfo>) -> c_int {
        0
    }

    #[doc = " Synchronize file contents"]
    fn fsync(
        &self,
        _path: String,
        _arg2: ::std::os::raw::c_int,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    fn setxattr(
        &self,
        _path: String,
        _name: String,
        _value: &[u8],
        _arg5: c_int,
        _arg6: u32,
    ) -> ::std::os::raw::c_int {
        -libc::ENOSYS
    }

    fn getxattr(&self, _path: String, _name: String, _value: &[u8], _arg5: u32) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " List extended attributes"]
    fn listxattr(&self, _path: String, _buf: &mut [u8]) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Remove extended attributes"]
    fn removexattr(&self, _path: String, _name: String) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Open directory"]
    fn opendir(&self, _path: String, _fi: Option<&mut FuseFileInfo>) -> c_int {
        0
    }

    #[doc = " Read directory"]
    fn readdir(
        &self,
        _path: String,
        _filler: ReadDirFiller,
        _offset: off_t,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Release directory"]
    fn releasedir(&self, _path: String, _fi: Option<&mut FuseFileInfo>) -> c_int {
        0
    }

    #[doc = " Synchronize directory contents"]
    fn fsyncdir(
        &self,
        _path: String,
        _arg2: ::std::os::raw::c_int,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    // NOTE unsupported
    // #[doc = " Initialize filesystem"]
    // fn init(&self, _conn: *mut FuseConnectionInfo) -> *mut c_void {
    //     ptr::null_mut()
    // }

    // #[doc = " Clean up filesystem"]
    // fn destroy(&self, arg1: *mut ::std::os::raw::c_void) {
    // }

    #[doc = " Check file access permissions"]
    fn access(&self, _path: String, _mode: ::std::os::raw::c_int) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Create and open a file"]
    fn create(
        &self,
        _path: String,
        _mode: ::libc::mode_t,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Change the size of an open file"]
    fn ftruncate(
        &self,
        _path: String,
        _offset: ::libc::off_t,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Get attributes from an open file"]
    fn fgetattr(
        &self,
        _path: String,
        _stbuf: Option<&mut ::libc::stat>,
        _fi: Option<&mut FuseFileInfo>,
    ) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Perform POSIX file locking operation"]
    fn lock(
        &self,
        _path: String,
        _fi: Option<&mut FuseFileInfo>,
        _cmd: ::std::os::raw::c_int,
        _arg3: *mut ::libc::flock,
    ) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Change the access and modification times of a file with"]
    fn utimens(&self, _path: String, _tv: ::libc::timespec) -> c_int {
        -libc::ENOSYS
    }

    #[doc = " Map block index within file to block index within device"]
    fn bmap(&self, _path: String, _blocksize: usize, _idx: &mut u64) -> c_int {
        -libc::ENOSYS
    }
}

#[doc = "safety fuse_fill_dir"]
pub struct ReadDirFiller {
    buf: *mut ::std::os::raw::c_void,
    raw_filler: ::yarf_sys::fuse_fill_dir_t,
}

impl ReadDirFiller {
    pub fn new(
        buf: *mut ::std::os::raw::c_void,
        raw_filler: ::yarf_sys::fuse_fill_dir_t,
    ) -> ReadDirFiller {
        ReadDirFiller { buf, raw_filler }
    }

    #[doc = "fill directory info"]
    pub fn fill(
        &self,
        name: &str,
        stbuf: *const stat,
        offset: ::libc::off_t,
    ) -> ::std::os::raw::c_int {
        if let Some(func) = self.raw_filler {
            if let Ok(cname) = CString::new(name) {
                return unsafe { func(self.buf, cname.as_ptr(), stbuf, offset) };
            }
        }
        -libc::EIO
    }
}

#[doc = "main() for your FileSystem implementation"]
pub fn yarf_main(fs: Box<FileSystem>) -> i64 {
    let ops = FuseOperations {
        getattr: Some(getattr_proxy),
        readlink: Some(readlink_proxy),
        getdir: Some(getdir_proxy),
        mknod: Some(mknod_proxy),
        mkdir: Some(mkdir_proxy),
        unlink: Some(unlink_proxy),
        rmdir: Some(rmdir_proxy),
        symlink: Some(symlink_proxy),
        rename: Some(rename_proxy),
        link: Some(link_proxy),
        chmod: Some(chmod_proxy),
        chown: Some(chown_proxy),
        truncate: Some(truncate_proxy),
        utime: Some(utime_proxy),
        open: Some(open_proxy),
        read: Some(read_proxy),
        write: Some(write_proxy),
        statfs: Some(statfs_proxy),
        flush: Some(flush_proxy),
        release: Some(release_proxy),
        fsync: Some(fsync_proxy),
        setxattr: Some(setxattr_proxy),
        getxattr: Some(getxattr_proxy),
        listxattr: Some(listxattr_proxy),
        removexattr: Some(removexattr_proxy),
        opendir: Some(opendir_proxy),
        readdir: Some(readdir_proxy),
        releasedir: Some(releasedir_proxy),
        fsyncdir: Some(fsyncdir_proxy),
        init: None, // Unsupported because it overwrites private_data used to passing Rust value
        destroy: None, // Unsupported
        access: Some(access_proxy),
        create: Some(create_proxy),
        ftruncate: Some(ftruncate_proxy),
        fgetattr: Some(fgetattr_proxy),
        lock: Some(lock_proxy),
        utimens: Some(utimens_proxy),
        bmap: Some(bmap_proxy),
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

//
// Fuse Callback proxies
//

extern "C" fn getattr_proxy(path: *const c_char, stbuf: *mut stat) -> c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    // Trust libfuse not to pass invalid pointer
    let stbuf_ref = unsafe {
        libc::memset(stbuf as *mut c_void, 0, mem::size_of_val(&stbuf));
        stbuf.as_mut()
    };

    ops.getattr(rpath, stbuf_ref)
}

extern "C" fn readlink_proxy(
    path: *const ::std::os::raw::c_char,
    buf: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let sbuf = unsafe { slice::from_raw_parts_mut(buf as *mut u8, size) };

    ops.readlink(rpath, sbuf)
}

extern "C" fn getdir_proxy(
    path: *const ::std::os::raw::c_char,
    arg2: ::yarf_sys::fuse_dirh_t,
    arg3: ::yarf_sys::fuse_dirfil_t,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.getdir(rpath, arg2, arg3)
}

extern "C" fn mknod_proxy(
    path: *const ::std::os::raw::c_char,
    mode: ::libc::mode_t,
    dev: ::libc::dev_t,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.mknod(rpath, mode, dev)
}

extern "C" fn mkdir_proxy(
    path: *const ::std::os::raw::c_char,
    mode: ::libc::mode_t,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.mkdir(rpath, mode)
}

extern "C" fn unlink_proxy(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.unlink(rpath)
}

extern "C" fn rmdir_proxy(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.rmdir(rpath)
}

extern "C" fn symlink_proxy(
    path1: *const ::std::os::raw::c_char,
    path2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath1 = to_rust_str(path1);
    let rpath2 = to_rust_str(path2);

    ops.symlink(rpath1, rpath2)
}

extern "C" fn rename_proxy(
    old: *const ::std::os::raw::c_char,
    new: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let old_str = to_rust_str(old);
    let new_str = to_rust_str(new);

    ops.rename(old_str, new_str)
}

extern "C" fn link_proxy(
    path1: *const ::std::os::raw::c_char,
    path2: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath1 = to_rust_str(path1);
    let rpath2 = to_rust_str(path2);

    ops.link(rpath1, rpath2)
}

extern "C" fn chmod_proxy(
    path: *const ::std::os::raw::c_char,
    mode: ::libc::mode_t,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.chmod(rpath, mode)
}

extern "C" fn chown_proxy(
    path: *const ::std::os::raw::c_char,
    uid: ::libc::uid_t,
    gid: ::libc::gid_t,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.chown(rpath, uid, gid)
}

extern "C" fn truncate_proxy(
    path: *const ::std::os::raw::c_char,
    offset: ::libc::off_t,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.truncate(rpath, offset)
}

extern "C" fn utime_proxy(
    path: *const ::std::os::raw::c_char,
    utimbuf: *mut ::libc::utimbuf,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let utimbuf_ref = unsafe { utimbuf.as_mut() };

    ops.utime(rpath, utimbuf_ref)
}

extern "C" fn open_proxy(path: *const c_char, fi: *mut FuseFileInfo) -> c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.open(rpath, fi_ref)
}

extern "C" fn read_proxy(
    path: *const c_char,
    buf: *mut c_char,
    size: usize,
    offset: off_t,
    fi: *mut FuseFileInfo,
) -> c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let sbuf = unsafe { slice::from_raw_parts_mut(buf as *mut u8, size) };
    let fi_ref = unsafe { fi.as_mut() };

    ops.read(rpath, sbuf, offset, fi_ref)
}

extern "C" fn write_proxy(
    path: *const ::std::os::raw::c_char,
    buf: *const ::std::os::raw::c_char,
    size: usize,
    offset: ::libc::off_t,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let sbuf = unsafe { slice::from_raw_parts(buf as *const u8, size) };
    let fi_ref = unsafe { fi.as_mut() };

    ops.write(rpath, sbuf, offset, fi_ref)
}

extern "C" fn statfs_proxy(
    path: *const ::std::os::raw::c_char,
    statvfs: *mut ::libc::statvfs,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let statvfs_ref = unsafe { statvfs.as_mut() };

    ops.statfs(rpath, statvfs_ref)
}

extern "C" fn flush_proxy(
    path: *const ::std::os::raw::c_char,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.flush(rpath, fi_ref)
}

extern "C" fn release_proxy(
    path: *const ::std::os::raw::c_char,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.release(rpath, fi_ref)
}

extern "C" fn fsync_proxy(
    path: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.fsync(rpath, arg2, fi_ref)
}

extern "C" fn setxattr_proxy(
    path: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
    size: usize,
    arg5: ::std::os::raw::c_int,
    arg6: u32,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let rname = to_rust_str(name);
    let svalue = unsafe { slice::from_raw_parts(value as *const u8, size) };

    ops.setxattr(rpath, rname, svalue, arg5, arg6)
}

extern "C" fn getxattr_proxy(
    path: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
    value: *mut ::std::os::raw::c_char,
    size: usize,
    arg5: u32,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let rname = to_rust_str(name);
    let svalue = unsafe { slice::from_raw_parts(value as *const u8, size) };

    ops.getxattr(rpath, rname, svalue, arg5)
}

extern "C" fn listxattr_proxy(
    path: *const ::std::os::raw::c_char,
    value: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let svalue = unsafe { slice::from_raw_parts_mut(value as *mut u8, size) };

    ops.listxattr(rpath, svalue)
}

extern "C" fn removexattr_proxy(
    path: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let rname = to_rust_str(name);

    ops.removexattr(rpath, rname)
}

extern "C" fn opendir_proxy(
    path: *const ::std::os::raw::c_char,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.opendir(rpath, fi_ref)
}

extern "C" fn readdir_proxy(
    path: *const c_char,
    buf: *mut c_void,
    filler: ::yarf_sys::fuse_fill_dir_t,
    offset: off_t,
    fi: *mut FuseFileInfo,
) -> c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let rich_filler = ReadDirFiller::new(buf, filler);
    let fi_ref = unsafe { fi.as_mut() };

    ops.readdir(rpath, rich_filler, offset, fi_ref)
}

extern "C" fn releasedir_proxy(
    path: *const ::std::os::raw::c_char,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.releasedir(rpath, fi_ref)
}

extern "C" fn fsyncdir_proxy(
    path: *const ::std::os::raw::c_char,
    arg2: ::std::os::raw::c_int,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.fsyncdir(rpath, arg2, fi_ref)
}

extern "C" fn access_proxy(
    path: *const ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);

    ops.access(rpath, mode)
}

extern "C" fn create_proxy(
    path: *const ::std::os::raw::c_char,
    mode: ::libc::mode_t,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.create(rpath, mode, fi_ref)
}

extern "C" fn ftruncate_proxy(
    path: *const ::std::os::raw::c_char,
    offset: ::libc::off_t,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.ftruncate(rpath, offset, fi_ref)
}

extern "C" fn fgetattr_proxy(
    path: *const ::std::os::raw::c_char,
    stbuf: *mut ::libc::stat,
    fi: *mut FuseFileInfo,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };
    let stbuf_ref = unsafe { stbuf.as_mut() };

    ops.fgetattr(rpath, stbuf_ref, fi_ref)
}

extern "C" fn lock_proxy(
    path: *const ::std::os::raw::c_char,
    fi: *mut FuseFileInfo,
    cmd: ::std::os::raw::c_int,
    arg3: *mut ::libc::flock,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let fi_ref = unsafe { fi.as_mut() };

    ops.lock(rpath, fi_ref, cmd, arg3)
}

extern "C" fn utimens_proxy(
    path: *const ::std::os::raw::c_char,
    tv: *const ::libc::timespec,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let rtv = unsafe { tv.as_ref().unwrap() };

    ops.utimens(rpath, *rtv)
}

extern "C" fn bmap_proxy(
    path: *const ::std::os::raw::c_char,
    blocksize: usize,
    idx: *mut u64,
) -> ::std::os::raw::c_int {
    let ops = unsafe { get_filesystem() };
    let rpath = to_rust_str(path);
    let ridx = unsafe { idx.as_mut().unwrap() };

    ops.bmap(rpath, blocksize, ridx)
}

//
// Utils
//

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
