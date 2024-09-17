use core::ffi::CStr;

use crate::{
    fs::{
        tmpfs::tmpfsdir,
        vfs::{get_list, vnode},
    },
    mem::phys::{align_up, RAMUSAGE},
    println,
    utils::get_limine_file,
};
extern crate alloc;
use alloc::string::*;

use super::vfs::{self, CUR_VFS};
// Stolen from the r3 kernel, credits to Narasimha Prasanna <3
#[inline]
fn oct2bin(s: &str) -> u32 {
    let mut n: u32 = 0;
    for u in s.chars() {
        n *= 8;
        let d = u as u8 - b'0';
        n += d as u32;
    }
    n
}
#[test]
fn test() {
    assert_eq!(53, oct2bin("65"));
    assert_eq!(85901285, oct2bin("507537745"))
}
#[test]
fn populate_vfs() {
    use std::fs;
    if !cfg!(target_os = "linux") {
        // not running this test
    } else {
        let mut o = fs::read("test.tar").unwrap();
        let mut e = o.as_mut_ptr() as *mut UStarHeader;
        let mut letskeepitnice = tmpfsdir::default();
        while (e as u64) < ((o.as_mut_ptr() as u64 + o.len() as u64) - 1024) {
            unsafe {
                match (*e).typeflag {
                    b'0' => {
                        std::println!("file found");

                        let okkkk = CStr::from_bytes_until_nul(&(*e).name)
                            .unwrap()
                            .to_str()
                            .unwrap();
                        std::println!(
                            "creating file with path {}",
                            get_list(okkkk).last().unwrap()
                        );
                        let ok = letskeepitnice
                            .create(get_list(okkkk).last().unwrap())
                            .unwrap();
                        let mut v = [0; 12];
                        v.copy_from_slice(&(*e).filesize);
                        let brooo =
                            oct2bin(CStr::from_bytes_until_nul(&v).unwrap().to_str().unwrap());
                        // let qqq = ok.borrow_mut().write(std::slice::from_raw_parts(
                        //     (e as u64 + 512) as *mut _,
                        //     brooo as usize
                        // ), 0).unwrap();
                        // assert_eq!(brooo as usize, qqq);
                        let mut buf = [0; 256];

                        // ok.borrow_mut().read(&mut buf, 0).unwrap();
                        std::println!("{:?}", buf);
                        let okk = CStr::from_bytes_until_nul(&buf).unwrap().to_str().unwrap();
                        std::println!("{}", okk);
                    }
                    b'1' => {
                        std::println!("hard link");
                    }
                    b'2' => {
                        std::println!("symbolic link")
                    }
                    b'3' => {
                        std::println!("character device");
                    }
                    b'4' => {
                        std::println!("block device");
                    }
                    b'5' => {
                        std::println!("directory");
                    }
                    b'6' => {
                        std::println!("pipe")
                    }
                    _ => {
                        std::println!("wtf");
                        return;
                    }
                }
            }
            unsafe {
                let mut v = [0; 12];
                v.copy_from_slice(&(*e).filesize);
                std::println!(
                    "{}",
                    oct2bin(CStr::from_bytes_until_nul(&v).unwrap().to_str().unwrap())
                );

                e = (e as u64
                    + 512
                    + align_up(
                        oct2bin(CStr::from_bytes_until_nul(&v).unwrap().to_str().unwrap()) as usize,
                        512,
                    ) as u64) as *mut UStarHeader;
            }
        }
    }
}
#[repr(C, packed)]
struct UStarHeader {
    name: [u8; 100],
    filemode: [u8; 8],
    ownerid: [u8; 8],
    groupid: [u8; 8],
    filesize: [u8; 12],
    unixtimemodifcation: [u8; 12],
    checksum: [u8; 8],
    typeflag: u8,
    linkedfile: [u8; 100],
    ustar: [u8; 6],
    ustarversion: [u8; 2],
    ownerusername: [u8; 32],
    ownergroupname: [u8; 32],
    devicemajor: [u8; 8],
    deviceminor: [u8; 8],
    filenameprefix: [u8; 155],
}
use alloc::rc::Rc;
use alloc::sync::Arc;
use spin::Mutex;
use core::cell::RefCell;
pub fn ustarinit() {
    let q = get_limine_file("initramfs");
    unsafe {
        CUR_VFS = Some(vfs::vfs::default());
        CUR_VFS.as_mut().unwrap().vnode = Some(Arc::new(Mutex::new(tmpfsdir::default())))
    }
    let mut cur_dir = unsafe { &mut CUR_VFS.as_mut().unwrap().vnode };
    if let Some(tar) = q {
        unsafe {
            println!("Ram Usage at the moment in bytes: {} bytes.", RAMUSAGE);
        }
        let mut e = tar.addr() as *mut UStarHeader;
        while (e as u64) < ((tar.addr() as u64 + tar.size()) - 1024)
        // dont ask why - 1024
        {
            unsafe {
                match (*e).typeflag {
                    b'0' => {
                        let okkkk = CStr::from_bytes_until_nul(&(*e).name)
                            .unwrap()
                            .to_str()
                            .unwrap();
                        println!("file found");
                        println!(
                            "creating file with name {}",
                            get_list(okkkk).last().unwrap()
                        );
                        for i in get_list(okkkk) {
                            let res = cur_dir.as_mut().unwrap().lock().lookup(i);
                            if let Ok(ress) = res {
                                *cur_dir = Some(ress);
                            }
                        }
                        if let Ok(gotyou) = cur_dir.as_mut().unwrap().lock().create(okkkk) {
                            println!("made file! writing data...");
                            let mut v = [0; 12];
                            v.copy_from_slice(&(*e).filesize);
                            let brooo =
                                oct2bin(CStr::from_bytes_until_nul(&v).unwrap().to_str().unwrap());
                            // let killme = gotyou.borrow_mut().write(
                            //     core::slice::from_raw_parts(
                            //         (e as u64 + 512) as *mut _,
                            //         brooo as usize
                            //     ), 0).unwrap();
                            // println!("wrote data of {} bytes.!", killme);
                        }
                    }
                    b'1' => {
                        println!("hard link");
                    }
                    b'2' => {
                        // let okkkk = CStr::from_bytes_until_nul(&(*e).name).unwrap().to_str().unwrap();
                        // println!("symlink found");
                        // println!("creating symlink with name {}", get_list(okkkk).last().unwrap());
                        // for i in get_list(okkkk) {
                        //     let res = cur_dir.as_mut().unwrap().borrow_mut().lookup(i);
                        //     if let Ok(ress) = res {
                        //         *cur_dir = Some(ress);
                        //     }
                        // }
                        // if let Ok(gotyou) = cur_dir.as_mut().unwrap().borrow_mut().create(okkkk) {
                        //     println!("made symlink! writing data...");
                        //     let mut v = [0; 12];
                        //     v.copy_from_slice(&(*e).filesize);
                        //     let ches = CStr::from_bytes_until_nul(&(*e).linkedfile).unwrap().to_str().unwrap();
                        //     println!("symlink {}", ches);
                        //     let brooo = oct2bin(CStr::from_bytes_until_nul(&v).unwrap().to_str().unwrap());
                        //     let killme = gotyou.borrow_mut().write(
                        //         &((*e).linkedfile)[..ches.len()], 0).unwrap();
                        //     gotyou.borrow_mut().set_symlink(true).unwrap();
                        //     println!("wrote data of {} bytes.!", killme);
                    }
                    b'3' => {
                        println!("character device");
                    }
                    b'4' => {
                        println!("block device");
                    }
                    b'5' => {
                        println!("directory");
                        let okkkk = CStr::from_bytes_until_nul(&(*e).name)
                            .unwrap()
                            .to_str()
                            .unwrap();
                        println!(
                            "creating directory with name {}",
                            get_list(okkkk).last().unwrap()
                        );
                        for i in get_list(okkkk) {
                            let res = cur_dir.as_mut().unwrap().lock().lookup(i);
                            if let Ok(ress) = res {
                                *cur_dir = Some(ress);
                            }
                        }
                        cur_dir.as_mut().unwrap().lock().mkdir(okkkk).unwrap();
                    }
                    b'6' => {
                        println!("pipe")
                    }
                    _ => {
                        return;
                    }
                }
                let mut v = [0; 12];
                v.copy_from_slice(&(*e).filesize);
                e = (e as u64
                    + 512
                    + align_up(
                        oct2bin(CStr::from_bytes_until_nul(&v).unwrap().to_str().unwrap()) as usize,
                        512,
                    ) as u64) as *mut UStarHeader;
            }
        }
        println!("all done!");
    } else {
        println!("got no file.... KILLING MYSELF");
        panic!("wah");
    }
}
