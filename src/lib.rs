/*
 * A libfuse highlevel API Rust bindings.
 * It supports only libfuse 2.6.x+
 *
 */
extern crate libc;

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

#[doc = " Option description"]
#[doc = ""]
#[doc = " This structure describes a single option, and and action associated"]
#[doc = " with it, in case it matches."]
#[doc = ""]
#[doc = " More than one such match may occur, in which case the action for"]
#[doc = " each match is executed."]
#[doc = ""]
#[doc = " There are three possible actions in case of a match:"]
#[doc = ""]
#[doc = " i) An integer (int or unsigned) variable determined by \'offset\' is"]
#[doc = "    set to \'value\'"]
#[doc = ""]
#[doc = " ii) The processing function is called, with \'value\' as the key"]
#[doc = ""]
#[doc = " iii) An integer (any) or string (char *) variable determined by"]
#[doc = "    \'offset\' is set to the value of an option parameter"]
#[doc = ""]
#[doc = " \'offset\' should normally be either set to"]
#[doc = ""]
#[doc = "  - \'offsetof(struct foo, member)\'  actions i) and iii)"]
#[doc = ""]
#[doc = "  - -1\t\t\t      action ii)"]
#[doc = ""]
#[doc = " The \'offsetof()\' macro is defined in the <stddef.h> header."]
#[doc = ""]
#[doc = " The template determines which options match, and also have an"]
#[doc = " effect on the action.  Normally the action is either i) or ii), but"]
#[doc = " if a format is present in the template, then action iii) is"]
#[doc = " performed."]
#[doc = ""]
#[doc = " The types of templates are:"]
#[doc = ""]
#[doc = " 1) \"-x\", \"-foo\", \"--foo\", \"--foo-bar\", etc.\tThese match only"]
#[doc = "   themselves.  Invalid values are \"--\" and anything beginning"]
#[doc = "   with \"-o\""]
#[doc = ""]
#[doc = " 2) \"foo\", \"foo-bar\", etc.  These match \"-ofoo\", \"-ofoo-bar\" or"]
#[doc = "    the relevant option in a comma separated option list"]
#[doc = ""]
#[doc = " 3) \"bar=\", \"--foo=\", etc.  These are variations of 1) and 2)"]
#[doc = "    which have a parameter"]
#[doc = ""]
#[doc = " 4) \"bar=%s\", \"--foo=%lu\", etc.  Same matching as above but perform"]
#[doc = "    action iii)."]
#[doc = ""]
#[doc = " 5) \"-x \", etc.  Matches either \"-xparam\" or \"-x param\" as"]
#[doc = "    two separate arguments"]
#[doc = ""]
#[doc = " 6) \"-x %s\", etc.  Combination of 4) and 5)"]
#[doc = ""]
#[doc = " If the format is \"%s\", memory is allocated for the string unlike"]
#[doc = " with scanf()."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fuse_opt {
    #[doc = " Matching template and optional parameter formatting"]
    pub templ: *const ::std::os::raw::c_char,
    #[doc = " Offset of variable within \'data\' parameter of fuse_opt_parse()"]
    #[doc = " or -1"]
    pub offset: ::std::os::raw::c_ulong,
    #[doc = " Value to set the variable to, or to be passed as \'key\' to the"]
    #[doc = " processing function.\t Ignored if template has a format"]
    pub value: ::std::os::raw::c_int,
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
        stbuf: *const ::libc::stat,
        off: ::libc::off_t,
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
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct fuse_dirhandle {
    _unused: [u8; 0],
}

#[allow(non_camel_case_types)]
pub type fuse_dirh_t = *mut fuse_dirhandle;

#[allow(non_camel_case_types)]
pub type fuse_dirfil_t = ::std::option::Option<
    unsafe extern "C" fn(
        h: fuse_dirh_t,
        name: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
        ino: ::libc::ino_t,
    ) -> ::std::os::raw::c_int,
>;

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct setattr_x {
    pub valid: i32,
    pub mode: ::libc::mode_t,
    pub uid: ::libc::uid_t,
    pub gid: ::libc::gid_t,
    pub size: ::libc::off_t,
    pub acctime: ::libc::timespec,
    pub modtime: ::libc::timespec,
    pub crtime: ::libc::timespec,
    pub chgtime: ::libc::timespec,
    pub bkuptime: ::libc::timespec,
    pub flags: u32,
}

#[doc = " The file system operations:"]
#[doc = ""]
#[doc = " Most of these should work very similarly to the well known UNIX"]
#[doc = " file system operations.  A major exception is that instead of"]
#[doc = " returning an error in \'errno\', the operation should return the"]
#[doc = " negated error value (-errno) directly."]
#[doc = ""]
#[doc = " All methods are optional, but some are essential for a useful"]
#[doc = " filesystem (e.g. getattr).  Open, flush, release, fsync, opendir,"]
#[doc = " releasedir, fsyncdir, access, create, ftruncate, fgetattr, lock,"]
#[doc = " init and destroy are special purpose methods, without which a full"]
#[doc = " featured filesystem can still be implemented."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
            stbuf: *mut ::libc::stat,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Read the target of a symbolic link"]
    #[doc = ""]
    #[doc = " The buffer should be filled with a null terminated string.  The"]
    #[doc = " buffer size argument includes the space for the terminating"]
    #[doc = " null character.\tIf the linkname is too long to fit in the"]
    #[doc = " buffer, it should be truncated.\tThe return value should be 0"]
    #[doc = " for success."]
    pub readlink: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_char,
            arg3: usize,
        ) -> ::std::os::raw::c_int,
    >,

    pub getdir: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: fuse_dirh_t,
            arg3: fuse_dirfil_t,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Create a file node"]
    #[doc = ""]
    #[doc = " This is called for creation of all non-directory, non-symlink"]
    #[doc = " nodes.  If the filesystem defines a create() method, then for"]
    #[doc = " regular files that will be called instead."]
    pub mknod: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::libc::mode_t,
            arg3: ::libc::dev_t,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Create a directory"]
    pub mkdir: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::libc::mode_t,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Remove a file"]
    pub unlink: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,

    #[doc = " Remove a directory"]
    pub rmdir: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,

    #[doc = " Create a symbolic link"]
    pub symlink: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Rename a file"]
    pub rename: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Create a hard link to a file"]
    pub link: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Change the permission bits of a file"]
    pub chmod: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::libc::mode_t,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Change the owner and group of a file"]
    pub chown: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::libc::uid_t,
            arg3: ::libc::gid_t,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Change the size of a file"]
    pub truncate: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::libc::off_t,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Change the access and/or modification times of a file"]
    #[doc = ""]
    #[doc = " Deprecated, use utimens() instead."]
    pub utime: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::libc::utimbuf,
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
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut fuse_file_info,
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
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_char,
            arg3: usize,
            arg4: ::libc::off_t,
            arg5: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Write data to an open file"]
    #[doc = ""]
    #[doc = " Write should return exactly the number of bytes requested"]
    #[doc = " except on error.\t An exception to this is when the \'direct_io\'"]
    #[doc = " mount option is specified (see read operation)."]
    #[doc = ""]
    #[doc = " Changed in version 2.2"]
    pub write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
            arg4: ::libc::off_t,
            arg5: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Get file system statistics"]
    #[doc = ""]
    #[doc = " The \'f_frsize\', \'f_favail\', \'f_fsid\' and \'f_flag\' fields are ignored"]
    #[doc = ""]
    #[doc = " Replaced \'struct statfs\' parameter with \'struct statvfs\' in"]
    #[doc = " version 2.5"]
    pub statfs: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::libc::statvfs,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Possibly flush cached data"]
    #[doc = ""]
    #[doc = " BIG NOTE: This is not equivalent to fsync().  It\'s not a"]
    #[doc = " request to sync dirty data."]
    #[doc = ""]
    #[doc = " Flush is called on each close() of a file descriptor.  So if a"]
    #[doc = " filesystem wants to return write errors in close() and the file"]
    #[doc = " has cached dirty data, this is a good place to write back data"]
    #[doc = " and return any errors.  Since many applications ignore close()"]
    #[doc = " errors this is not always useful."]
    #[doc = ""]
    #[doc = " NOTE: The flush() method may be called more than once for each"]
    #[doc = " open().\tThis happens if more than one file descriptor refers"]
    #[doc = " to an opened file due to dup(), dup2() or fork() calls.\tIt is"]
    #[doc = " not possible to determine if a flush is final, so each flush"]
    #[doc = " should be treated equally.  Multiple write-flush sequences are"]
    #[doc = " relatively rare, so this shouldn\'t be a problem."]
    #[doc = ""]
    #[doc = " Filesystems shouldn\'t assume that flush will always be called"]
    #[doc = " after some writes, or that if will be called at all."]
    #[doc = ""]
    #[doc = " Changed in version 2.2"]
    pub flush: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Release an open file"]
    #[doc = ""]
    #[doc = " Release is called when there are no more references to an open"]
    #[doc = " file: all file descriptors are closed and all memory mappings"]
    #[doc = " are unmapped."]
    #[doc = ""]
    #[doc = " For every open() call there will be exactly one release() call"]
    #[doc = " with the same flags and file descriptor.\t It is possible to"]
    #[doc = " have a file opened more than once, in which case only the last"]
    #[doc = " release will mean, that no more reads/writes will happen on the"]
    #[doc = " file.  The return value of release is ignored."]
    #[doc = ""]
    #[doc = " Changed in version 2.2"]
    pub release: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Synchronize file contents"]
    #[doc = ""]
    #[doc = " If the datasync parameter is non-zero, then only the user data"]
    #[doc = " should be flushed, not the meta data."]
    #[doc = ""]
    #[doc = " Changed in version 2.2"]
    pub fsync: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
            arg3: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    pub setxattr: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *const ::std::os::raw::c_char,
            arg4: usize,
            arg5: ::std::os::raw::c_int,
            arg6: u32,
        ) -> ::std::os::raw::c_int,
    >,

    pub getxattr: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: *mut ::std::os::raw::c_char,
            arg4: usize,
            arg5: u32,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " List extended attributes"]
    pub listxattr: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_char,
            arg3: usize,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Remove extended attributes"]
    pub removexattr: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Open directory"]
    #[doc = ""]
    #[doc = " This method should check if the open operation is permitted for"]
    #[doc = " this  directory"]
    #[doc = ""]
    #[doc = " Introduced in version 2.3"]
    pub opendir: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
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
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_void,
            arg3: fuse_fill_dir_t,
            arg4: ::libc::off_t,
            arg5: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Release directory"]
    #[doc = ""]
    #[doc = " Introduced in version 2.3"]
    pub releasedir: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Synchronize directory contents"]
    #[doc = ""]
    #[doc = " If the datasync parameter is non-zero, then only the user data"]
    #[doc = " should be flushed, not the meta data"]
    #[doc = ""]
    #[doc = " Introduced in version 2.3"]
    pub fsyncdir: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
            arg3: *mut fuse_file_info,
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

    #[doc = " Clean up filesystem"]
    #[doc = ""]
    #[doc = " Called on filesystem exit."]
    #[doc = ""]
    #[doc = " Introduced in version 2.3"]
    pub destroy: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,

    #[doc = " Check file access permissions"]
    #[doc = ""]
    #[doc = " This will be called for the access() system call.  If the"]
    #[doc = " \'default_permissions\' mount option is given, this method is not"]
    #[doc = " called."]
    #[doc = ""]
    #[doc = " This method is not called under Linux kernel versions 2.4.x"]
    #[doc = ""]
    #[doc = " Introduced in version 2.5"]
    pub access: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Create and open a file"]
    #[doc = ""]
    #[doc = " If the file does not exist, first create it with the specified"]
    #[doc = " mode, and then open it."]
    #[doc = ""]
    #[doc = " If this method is not implemented or under Linux kernel"]
    #[doc = " versions earlier than 2.6.15, the mknod() and open() methods"]
    #[doc = " will be called instead."]
    #[doc = ""]
    #[doc = " Introduced in version 2.5"]
    pub create: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::libc::mode_t,
            arg3: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Change the size of an open file"]
    #[doc = ""]
    #[doc = " This method is called instead of the truncate() method if the"]
    #[doc = " truncation was invoked from an ftruncate() system call."]
    #[doc = ""]
    #[doc = " If this method is not implemented or under Linux kernel"]
    #[doc = " versions earlier than 2.6.15, the truncate() method will be"]
    #[doc = " called instead."]
    #[doc = ""]
    #[doc = " Introduced in version 2.5"]
    pub ftruncate: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: ::libc::off_t,
            arg3: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Get attributes from an open file"]
    #[doc = ""]
    #[doc = " This method is called instead of the getattr() method if the"]
    #[doc = " file information is available."]
    #[doc = ""]
    #[doc = " Currently this is only called after the create() method if that"]
    #[doc = " is implemented (see above).  Later it may be called for"]
    #[doc = " invocations of fstat() too."]
    #[doc = ""]
    #[doc = " Introduced in version 2.5"]
    pub fgetattr: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::libc::stat,
            arg3: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Perform POSIX file locking operation"]
    #[doc = ""]
    #[doc = " The cmd argument will be either F_GETLK, F_SETLK or F_SETLKW."]
    #[doc = ""]
    #[doc = " For the meaning of fields in \'struct flock\' see the man page"]
    #[doc = " for fcntl(2).  The l_whence field will always be set to"]
    #[doc = " SEEK_SET."]
    #[doc = ""]
    #[doc = " For checking lock ownership, the \'fuse_file_info->owner\'"]
    #[doc = " argument must be used."]
    #[doc = ""]
    #[doc = " For F_GETLK operation, the library will first check currently"]
    #[doc = " held locks, and if a conflicting lock is found it will return"]
    #[doc = " information without calling this method.\t This ensures, that"]
    #[doc = " for local locks the l_pid field is correctly filled in.\tThe"]
    #[doc = " results may not be accurate in case of race conditions and in"]
    #[doc = " the presence of hard links, but it\'s unlikly that an"]
    #[doc = " application would rely on accurate GETLK results in these"]
    #[doc = " cases.  If a conflicting lock is not found, this method will be"]
    #[doc = " called, and the filesystem may fill out l_pid by a meaningful"]
    #[doc = " value, or it may leave this field zero."]
    #[doc = ""]
    #[doc = " For F_SETLK and F_SETLKW the l_pid field will be set to the pid"]
    #[doc = " of the process performing the locking operation."]
    #[doc = ""]
    #[doc = " Note: if this method is not implemented, the kernel will still"]
    #[doc = " allow file locking to work locally.  Hence it is only"]
    #[doc = " interesting for network filesystems and similar."]
    #[doc = ""]
    #[doc = " Introduced in version 2.6"]
    pub lock: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut fuse_file_info,
            cmd: ::std::os::raw::c_int,
            arg3: *mut ::libc::flock,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Change the access and modification times of a file with"]
    #[doc = " nanosecond resolution"]
    #[doc = ""]
    #[doc = " Introduced in version 2.6"]
    pub utimens: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            tv: *const ::libc::timespec,
        ) -> ::std::os::raw::c_int,
    >,

    #[doc = " Map block index within file to block index within device"]
    #[doc = ""]
    #[doc = " Note: This makes sense only for block device backed filesystems"]
    #[doc = " mounted with the \'blkdev\' option"]
    #[doc = ""]
    #[doc = " Introduced in version 2.6"]
    pub bmap: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            blocksize: usize,
            idx: *mut u64,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved00: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved01: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved02: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved03: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved04: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved05: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved06: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved07: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved08: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved09: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub reserved10: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_void,
            arg3: *mut ::std::os::raw::c_void,
            arg4: *mut ::std::os::raw::c_void,
            arg5: *mut ::std::os::raw::c_void,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *mut ::std::os::raw::c_void,
            arg8: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,

    pub setvolname: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int,
    >,

    pub exchange: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_ulong,
        ) -> ::std::os::raw::c_int,
    >,

    pub getxtimes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            bkuptime: *mut ::libc::timespec,
            crtime: *mut ::libc::timespec,
        ) -> ::std::os::raw::c_int,
    >,

    pub setbkuptime: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            tv: *const ::libc::timespec,
        ) -> ::std::os::raw::c_int,
    >,

    pub setchgtime: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            tv: *const ::libc::timespec,
        ) -> ::std::os::raw::c_int,
    >,

    pub setcrtime: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            tv: *const ::libc::timespec,
        ) -> ::std::os::raw::c_int,
    >,

    pub chflags: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: u32,
        ) -> ::std::os::raw::c_int,
    >,

    pub setattr_x: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut setattr_x,
        ) -> ::std::os::raw::c_int,
    >,

    pub fsetattr_x: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut setattr_x,
            arg3: *mut fuse_file_info,
        ) -> ::std::os::raw::c_int,
    >,
}

extern "C" {
    #[doc = " The real main function"]
    #[doc = ""]
    #[doc = " Do not call this directly, use fuse_main()"]
    #[link_name = "fuse_main_real"]
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
    #[link_name = "fuse_opt_parse"]
    pub fn fuse_opt_parse(
        args: *mut fuse_args,
        data: *mut ::std::os::raw::c_void,
        opts: *const fuse_opt,
        proc_: fuse_opt_proc_t,
    ) -> ::std::os::raw::c_int;

    #[doc = " Add an option to a comma separated option list"]
    #[doc = ""]
    #[doc = " @param opts is a pointer to an option list, may point to a NULL value"]
    #[doc = " @param opt is the option to add"]
    #[doc = " @return -1 on allocation error, 0 on success"]
    #[link_name = "fuse_opt_add_opt"]
    pub fn fuse_opt_add_opt(
        opts: *mut *mut ::std::os::raw::c_char,
        opt: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    #[doc = " Add an argument to a NULL terminated argument vector"]
    #[doc = ""]
    #[doc = " @param args is the structure containing the current argument list"]
    #[doc = " @param arg is the new argument to add"]
    #[doc = " @return -1 on allocation error, 0 on success"]
    #[link_name = "fuse_opt_add_arg"]
    pub fn fuse_opt_add_arg(
        args: *mut fuse_args,
        arg: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    #[doc = " Add an argument at the specified position in a NULL terminated"]
    #[doc = " argument vector"]
    #[doc = ""]
    #[doc = " Adds the argument to the N-th position.  This is useful for adding"]
    #[doc = " options at the beggining of the array which must not come after the"]
    #[doc = " special \'--\' option."]
    #[doc = ""]
    #[doc = " @param args is the structure containing the current argument list"]
    #[doc = " @param pos is the position at which to add the argument"]
    #[doc = " @param arg is the new argument to add"]
    #[doc = " @return -1 on allocation error, 0 on success"]
    #[link_name = "fuse_opt_insert_arg"]
    pub fn fuse_opt_insert_arg(
        args: *mut fuse_args,
        pos: ::std::os::raw::c_int,
        arg: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    #[doc = " Free the contents of argument list"]
    #[doc = ""]
    #[doc = " The structure itself is not freed"]
    #[doc = ""]
    #[doc = " @param args is the structure containing the argument list"]
    #[link_name = "fuse_opt_free_args"]
    pub fn fuse_opt_free_args(args: *mut fuse_args);

    #[doc = " Check if an option matches"]
    #[doc = ""]
    #[doc = " @param opts is the option description array"]
    #[doc = " @param opt is the option to match"]
    #[doc = " @return 1 if a match is found, 0 if not"]
    #[link_name = "fuse_opt_match"]
    pub fn fuse_opt_match(
        opts: *const fuse_opt,
        opt: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}

#[test]
fn bindgen_test_layout_fuse_operations() {
    assert_eq!(
        ::std::mem::size_of::<fuse_operations>(),
        464usize,
        concat!("Size of: ", stringify!(fuse_operations))
    );
    assert_eq!(
        ::std::mem::align_of::<fuse_operations>(),
        8usize,
        concat!("Alignment of ", stringify!(fuse_operations))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).getattr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(getattr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).readlink as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(readlink)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).getdir as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(getdir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).mknod as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(mknod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).mkdir as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(mkdir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).unlink as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(unlink)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).rmdir as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(rmdir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).symlink as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(symlink)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).rename as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(rename)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).link as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(link)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).chmod as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(chmod)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).chown as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(chown)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).truncate as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(truncate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).utime as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(utime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).open as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(open)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).read as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(read)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).write as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(write)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).statfs as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(statfs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).flush as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(flush)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).release as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(release)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).fsync as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(fsync)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).setxattr as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(setxattr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).getxattr as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(getxattr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).listxattr as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(listxattr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).removexattr as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(removexattr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).opendir as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(opendir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).readdir as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(readdir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).releasedir as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(releasedir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).fsyncdir as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(fsyncdir)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).init as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).destroy as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(destroy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).access as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(access)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).create as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(create)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).ftruncate as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(ftruncate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).fgetattr as *const _ as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(fgetattr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).lock as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).utimens as *const _ as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(utimens)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).bmap as *const _ as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(bmap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved00 as *const _ as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved00)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved01 as *const _ as usize },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved01)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved02 as *const _ as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved02)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved03 as *const _ as usize },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved03)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved04 as *const _ as usize },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved04)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved05 as *const _ as usize },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved05)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved06 as *const _ as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved06)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved07 as *const _ as usize },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved07)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved08 as *const _ as usize },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved08)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved09 as *const _ as usize },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved09)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).reserved10 as *const _ as usize },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(reserved10)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).setvolname as *const _ as usize },
        392usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(setvolname)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).exchange as *const _ as usize },
        400usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(exchange)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).getxtimes as *const _ as usize },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(getxtimes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).setbkuptime as *const _ as usize },
        416usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(setbkuptime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).setchgtime as *const _ as usize },
        424usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(setchgtime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).setcrtime as *const _ as usize },
        432usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(setcrtime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).chflags as *const _ as usize },
        440usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(chflags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).setattr_x as *const _ as usize },
        448usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(setattr_x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<fuse_operations>())).fsetattr_x as *const _ as usize },
        456usize,
        concat!(
            "Offset of field: ",
            stringify!(fuse_operations),
            "::",
            stringify!(fsetattr_x)
        )
    );
}
