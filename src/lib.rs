extern crate libc;

use libc::stat;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

#[allow(non_camel_case_types)]
pub type off_t = ::std::os::raw::c_longlong;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct __BindgenBitfieldUnit<Storage, Align>
    where
        Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

#[doc = " Argument list"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fuse_args {
    #[doc = " Argument count"]
    pub argc: ::std::os::raw::c_int,
    #[doc = " Argument vector.  NULL terminated"]
    pub argv: *mut *mut ::std::os::raw::c_char,
    #[doc = " Is \'argv\' allocated?"]
    pub allocated: ::std::os::raw::c_int,
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct fuse_opt {
    templ: *const c_char,
    offset: c_ulong,
    value: c_int,
}

#[doc = " Connection information, passed to the ->init() method"]
#[doc = ""]
#[doc = " Some of the elements are read-write, these can be changed to"]
#[doc = " indicate the value requested by the filesystem.  The requested"]
#[doc = " value must usually be smaller than the indicated value."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fuse_conn_info {
    #[doc = " Major version of the protocol (read-only)"]
    pub proto_major: ::std::os::raw::c_uint,
    #[doc = " Minor version of the protocol (read-only)"]
    pub proto_minor: ::std::os::raw::c_uint,
    #[doc = " Is asynchronous read supported (read-write)"]
    pub async_read: ::std::os::raw::c_uint,
    #[doc = " Maximum size of the write buffer"]
    pub max_write: ::std::os::raw::c_uint,
    #[doc = " Maximum readahead"]
    pub max_readahead: ::std::os::raw::c_uint,
    // TODO pub enable: fuse_conn_info__bindgen_ty_1,
    pub reserved: [::std::os::raw::c_uint; 26usize],
}

#[doc = " Information about open files"]
#[doc = ""]
#[doc = " Changed in version 2.5"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fuse_file_info {
    #[doc = " Open flags.\t Available in open() and release()"]
    pub flags: ::std::os::raw::c_int,
    #[doc = " Old file handle, don\'t use"]
    pub fh_old: ::std::os::raw::c_ulong,
    #[doc = " In case of a write operation indicates if this was caused by a"]
    #[doc = "writepage"]
    pub writepage: ::std::os::raw::c_int,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u32>,
    #[doc = " File handle.  May be filled in by filesystem in open()."]
    #[doc = "Available in all other file operations"]
    pub fh: u64,
    #[doc = " Lock owner id.  Available in locking operations and flush"]
    pub lock_owner: u64,
}

#[doc = " Function to add an entry in a readdir() operation"]
#[doc = ""]
#[doc = " @param buf the buffer passed to the readdir() operation"]
#[doc = " @param name the file name of the directory entry"]
#[doc = " @param stat file attributes, can be NULL"]
#[doc = " @param off offset of the next entry or zero"]
#[doc = " @return 1 if buffer is full, zero otherwise"]
#[allow(non_camel_case_types)]
pub type fuse_fill_dir_t = ::std::option::Option<
    unsafe extern "C" fn(
        buf: *mut ::std::os::raw::c_void,
        name: *const ::std::os::raw::c_char,
        stbuf: *const stat,
        off: off_t,
    ) -> ::std::os::raw::c_int,
>;

#[doc = " Processing function"]
#[doc = ""]
#[doc = " This function is called if"]
#[doc = "    - option did not match any \'struct fuse_opt\'"]
#[doc = "    - argument is a non-option"]
#[doc = "    - option did match and offset was set to -1"]
#[doc = ""]
#[doc = " The \'arg\' parameter will always contain the whole argument or"]
#[doc = " option including the parameter if exists.  A two-argument option"]
#[doc = " (\"-x foo\") is always converted to single arguemnt option of the"]
#[doc = " form \"-xfoo\" before this function is called."]
#[doc = ""]
#[doc = " Options of the form \'-ofoo\' are passed to this function without the"]
#[doc = " \'-o\' prefix."]
#[doc = ""]
#[doc = " The return value of this function determines whether this argument"]
#[doc = " is to be inserted into the output argument vector, or discarded."]
#[doc = ""]
#[doc = " @param data is the user data passed to the fuse_opt_parse() function"]
#[doc = " @param arg is the whole argument or option"]
#[doc = " @param key determines why the processing function was called"]
#[doc = " @param outargs the current output argument list"]
#[doc = " @return -1 on error, 0 if arg is to be discarded, 1 if arg should be kept"]
#[allow(non_camel_case_types)]
pub type fuse_opt_proc_t = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut ::std::os::raw::c_void,
        arg: *const ::std::os::raw::c_char,
        key: ::std::os::raw::c_int,
        outargs: *mut fuse_args,
    ) -> ::std::os::raw::c_int,
>;

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct fuse_operations {
    #[doc = " Get file attributes."]
    #[doc = ""]
    #[doc = " Similar to stat().  The \'st_dev\' and \'st_blksize\' fields are"]
    #[doc = " ignored.\t The \'st_ino\' field is ignored except if the \'use_ino\'"]
    #[doc = " mount option is given."]
    pub getattr: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            stbuf: *mut stat,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Initialize filesystem"]
    #[doc = ""]
    #[doc = " The return value will passed in the private_data field of"]
    #[doc = " fuse_context to all file operations and as a parameter to the"]
    #[doc = " destroy() method."]
    #[doc = ""]
    #[doc = " Introduced in version 2.3"]
    #[doc = " Changed in version 2.6"]
    pub init: ::std::option::Option<
        unsafe extern "C" fn(conn: *mut fuse_conn_info) -> *mut ::std::os::raw::c_void,
    >,

    #[doc = " Read directory"]
    #[doc = ""]
    #[doc = " This supersedes the old getdir() interface.  New applications"]
    #[doc = " should use this."]
    #[doc = ""]
    #[doc = " The filesystem may choose between two modes of operation:"]
    #[doc = ""]
    #[doc = " 1) The readdir implementation ignores the offset parameter, and"]
    #[doc = " passes zero to the filler function\'s offset.  The filler"]
    #[doc = " function will not return \'1\' (unless an error happens), so the"]
    #[doc = " whole directory is read in a single readdir operation.  This"]
    #[doc = " works just like the old getdir() method."]
    #[doc = ""]
    #[doc = " 2) The readdir implementation keeps track of the offsets of the"]
    #[doc = " directory entries.  It uses the offset parameter and always"]
    #[doc = " passes non-zero offset to the filler function.  When the buffer"]
    #[doc = " is full (or an error happens) the filler function will return"]
    #[doc = " \'1\'."]
    #[doc = ""]
    #[doc = " Introduced in version 2.3"]
    pub readdir: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            buf: *mut ::std::os::raw::c_void,
            filler: fuse_fill_dir_t,
            offset: off_t,
            fi: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " File open operation"]
    #[doc = ""]
    #[doc = " No creation, or truncation flags (O_CREAT, O_EXCL, O_TRUNC)"]
    #[doc = " will be passed to open().  Open should check if the operation"]
    #[doc = " is permitted for the given flags.  Optionally open may also"]
    #[doc = " return an arbitrary filehandle in the fuse_file_info structure,"]
    #[doc = " which will be passed to all file operations."]
    #[doc = ""]
    #[doc = " Changed in version 2.2"]
    pub open: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            fi: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Read data from an open file"]
    #[doc = ""]
    #[doc = " Read should return exactly the number of bytes requested except"]
    #[doc = " on EOF or error, otherwise the rest of the data will be"]
    #[doc = " substituted with zeroes.\t An exception to this is when the"]
    #[doc = " \'direct_io\' mount option is specified, in which case the return"]
    #[doc = " value of the read system call will reflect the return value of"]
    #[doc = " this operation."]
    #[doc = ""]
    #[doc = " Changed in version 2.2"]
    pub read: ::std::option::Option<
        unsafe extern "C" fn(
            path: *const ::std::os::raw::c_char,
            buf: *mut ::std::os::raw::c_char,
            size: usize,
            offset: off_t,
            fi: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,
}

extern "C" {
    #[doc = " The real main function"]
    #[doc = ""]
    #[doc = " Do not call this directly, use fuse_main()"]
    #[link_name = "\u{1}_fuse_main_real"]
    pub fn fuse_main_real(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
        op: *const fuse_operations,
        op_size: usize,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    #[doc = " Option parsing function"]
    #[doc = ""]
    #[doc = " If \'args\' was returned from a previous call to fuse_opt_parse() or"]
    #[doc = " it was constructed from"]
    #[doc = ""]
    #[doc = " A NULL \'args\' is equivalent to an empty argument vector"]
    #[doc = ""]
    #[doc = " A NULL \'opts\' is equivalent to an \'opts\' array containing a single"]
    #[doc = " end marker"]
    #[doc = ""]
    #[doc = " A NULL \'proc\' is equivalent to a processing function always"]
    #[doc = " returning \'1\'"]
    #[doc = ""]
    #[doc = " @param args is the input and output argument list"]
    #[doc = " @param data is the user data"]
    #[doc = " @param opts is the option description array"]
    #[doc = " @param proc is the processing function"]
    #[doc = " @return -1 on error, 0 on success"]
    #[link_name = "\u{1}_fuse_opt_parse"]
    pub fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut ::std::os::raw::c_void,
        opts: *const fuse_opt,
        proc_: fuse_opt_proc_t,
    ) -> ::std::os::raw::c_int;
}
