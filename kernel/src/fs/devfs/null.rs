
use crate::fs::tmpfs::tmpfsdir;
use crate::fs::vfs;
use crate::fs::vfs::CUR_VFS;
use crate::println;

use super::devfsops;
use super::devfsdir;
use super::devfsfile;
use super::devfsnode;

pub struct NullDriver;

impl devfsops for NullDriver {
    fn read(&self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, crate::utils::UNIXERROR> {
        Ok(0)
    }
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, crate::utils::UNIXERROR> {
        Ok(count)
    }
}
