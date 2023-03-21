// // #[derive(Debug,Default)]
// // struct Person{
// //     name: String,
// //     age: u8
// // }

// // fn main(){
// //     let mut p1 = Person::default();
// //     println!("{p1:?}");
// // }
// // use std::io::{BufRead, BufReader, Read, Result, Write};

// // fn count_lines<R: Read>(reader: R) -> usize {
// //     let buf_reader = BufReader::new(reader);
// //     buf_reader.lines().count()
// // }

// // fn main() -> Result<()> {
// //     let slice: &[u8] = b"foo\nbar\nbaz\n";
// //     println!("lines in slice: {}", count_lines(slice));
// //     let file = std::fs::File::open(std::env::current_exe()?)?;
// //     println!("lines in file: {}", count_lines(file));

// //     Ok(())
// // }
// // use std::io::{Result, Write};

// // fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
// //     writer.write_all(msg.as_bytes())?;
// //     writer.write_all("\n".as_bytes())
// // }

// // fn main() -> Result<()> {
// //     let mut buffer = Vec::new();
// //     log(&mut buffer, "Hello")?;
// //     log(&mut buffer, "World")?;
// //     println!("Logged: {:?}", String::from_utf8_lossy(&buffer));
// //     Ok(())
// // }
// // struct Droppable {
// //     name: &'static str,
// // }

// // impl Drop for Droppable {
// //     fn drop(&mut self) {
// //         println!("Dropping {}", self.name);
// //     }
// // }

// // fn main() {
// //     let a = Droppable { name: "a" };
// //     {
// //         let b = Droppable { name: "b" };
// //         {
// //             let c = Droppable { name: "c" };
// //             let d = Droppable { name: "d" };
// //             println!("Exiting block B");
// //         }
// //         println!("Exiting block A");
// //     }
// //     drop(a);
// //     // a.drop();
// //     println!("Exiting main");
// // }
// #[derive(Debug, Default)]
// struct Derived {
//     x: u32,
//     y: String,
//     z: Implemented,
// }

// #[derive(Debug)]
// struct Implemented(String);

// impl Default for Implemented {
//     fn default() -> Self {
//         Self("John Smith".into())
//     }
// }

// fn main() {
//     let default_struct: Derived = Default::default();
//     println!("{default_struct:#?}");

//     let almost_default_struct = Derived {
//         y: "Y is set!".into(),
//         ..Default::default()
//     };
//     println!("{almost_default_struct:#?}");

//     let nothing: Option<Derived> = None;
//     println!("{:#?}", nothing.unwrap_or_default());
// }
fn main() {
    let width = 15;
    println!("left aligned:  +{:-<width$}+", "");
    println!("centered:      |{: ^width$}|", "click me!");
    println!("right aligned: +{:-<width$}+", " ");
}
// TODO: remove this when you're done with your implementation.
// #![allow(unused_imports, unused_variables, dead_code)]

// mod ffi {
//     use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

//     // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
//     #[repr(C)]
//     pub struct DIR {
//         _data: [u8; 0],
//         _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
//     }

//     // Layout as per readdir(3) and definitions in /usr/include/x86_64-linux-gnu.
//     #[repr(C)]
//     pub struct dirent {
//         pub d_ino: c_long,
//         pub d_off: c_ulong,
//         pub d_reclen: c_ushort,
//         pub d_type: c_char,
//         pub d_name: [c_char; 256],
//     }

//     extern "C" {
//         pub fn opendir(s: *const c_char) -> *mut DIR;
//         pub fn readdir(s: *mut DIR) -> *const dirent;
//         pub fn closedir(s: *mut DIR) -> c_int;
//     }
// }

// use std::ffi::{CStr, CString, OsStr, OsString};
// use std::os::unix::ffi::OsStrExt;

// #[derive(Debug)]
// struct DirectoryIterator {
//     path: CString,
//     dir: *mut ffi::DIR,
// }

// impl DirectoryIterator {
//     fn new(path: &str) -> Result<DirectoryIterator, String> {
//         // Call opendir and return a Ok value if that worked,
//         // otherwise return Err with a message.
//         unimplemented!()
//     }
// }

// impl Iterator for DirectoryIterator {
//     type Item = OsString;
//     fn next(&mut self) -> Option<OsString> {
//         // Keep calling readdir until we get a NULL pointer back.
//         unimplemented!()
//     }
// }

// impl Drop for DirectoryIterator {
//     fn drop(&mut self) {
//         // Call closedir as needed.
//         unimplemented!()
//     }
// }

// fn main() -> Result<(), String> {
//     let iter = DirectoryIterator::new(".")?;
//     println!("files: {:#?}", iter.collect::<Vec<_>>());
//     Ok(())
// }
