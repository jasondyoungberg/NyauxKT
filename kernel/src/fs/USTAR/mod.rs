use crate::{mem::phys::RAMUSAGE, println, utils::get_limine_file};
extern crate alloc;
use alloc::string::*;
// Stolen from the r3 kernel, credits to Narasimha Prasanna <3
#[inline]
fn oct2bin(s: &[u8]) -> u32 {
	let mut n: u32 = 0;
	for u in s {
		n *= 8;
		let d = *u - b'0';
		n += d as u32;
	}
	n
}
#[test]
fn test() {
    assert_eq!(53 , oct2bin("65".as_bytes()));
    assert_eq!(85901285, oct2bin("507537745".as_bytes()))
}

#[repr(C, packed)]
struct UStarHeader
{
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
    filenameprefix: [u8; 155]
}

pub fn ustarinit()
{
    
    let q = get_limine_file("initramfs");
    if let Some(tar) = q
    {
        unsafe {
            println!("Ram Usage at the moment in bytes: {} bytes.", RAMUSAGE);
        }
        let mut e = tar.addr() as *mut UStarHeader;
        while (e as u64) < ((tar.addr() as u64 + tar.size()) - 1024) // dont ask why - 1024
        {
            
            unsafe {
                match (*e).typeflag
                {
                    b'0' => {
                        println!("file found");
                    },
                    b'1' => {
                        println!("hard link");
                    },
                    b'2' => {
                        println!("symbolic link")
                    },
                    b'3' => {
                        println!("character device");
                    },
                    b'4' => {
                        println!("block device");
                    },
                    b'5' => {
                        println!("directory");
                    },
                    b'6' => {
                        println!("pipe")
                    },
                    _ => {
                        println!("wtf");
                        return;
                    }
                }
            }
        }
    }
    else {
        println!("got no file.... KILLING MYSELF");
        panic!("wah");
    }
    
}