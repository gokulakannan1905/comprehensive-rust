// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout as per readdir(3) and definitions in /usr/include/x86_64-linux-gnu.
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_long,
        pub d_off: c_ulong,
        pub d_reclen: c_ushort,
        pub d_type: c_char,
        pub d_name: [c_char; 256],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

use ffi::{readdir, closedir};

use crate::ffi::opendir;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // or return err with proper message
        let path = CString::new(path).map_err(|e| format!("invalid path {e}"))?;
        
            let dir = unsafe{opendir(path.as_ptr())};
            if dir.is_null() {
                return Err(format!("Cannot open the directory {path:?}"));
            }else{
                return Ok(Self { path, dir });
            }       
        
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        
            let dp = unsafe{readdir(self.dir)};
            if dp.is_null(){
                return None;
            }
            let d_name = unsafe{
                CStr::from_ptr((*dp).d_name.as_ptr())
            };
            let os_str = OsStr::from_bytes(d_name.to_bytes());
            Some(os_str.into())
        
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        let exit_status = unsafe{ 
            if !self.dir.is_null(){
                closedir(self.dir)
            }else{
                0
            }
        };
        if exit_status != 0 {
            panic!("could not close the dir {:?}",self.path);
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new("/home")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}