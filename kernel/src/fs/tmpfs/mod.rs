use core::alloc::Layout;
use core::iter::Map;
use core::ptr::addr_of_mut;

use crate::utils::UNIXERROR;

extern crate alloc;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;
use spin::Mutex;
use alloc::string::String;
use alloc::string::String as privatetype;
use hashbrown::HashMap;

use super::vfs::{self, vnodeattributetable};
pub struct tmpfsdir {
    pub files: HashMap<String, Arc<Mutex<dyn vfs::vnode>>>,
    pub attrib: vnodeattributetable,
}
pub struct tmpfsfile {
    data: alloc::vec::Vec<u8>,
    attrib: vnodeattributetable,
}
impl Default for tmpfsdir {
    fn default() -> Self {
        Self {
            files: HashMap::new(),
            attrib: vnodeattributetable {
                size: 0,
                TYPE: super::VNODEFLAGS::DIR,
            },
        }
    }
}
impl<'a> Default for tmpfsfile {
    fn default() -> Self {
        Self {
            data: alloc::vec::Vec::new(),
            attrib: vnodeattributetable {
                size: 0,
                TYPE: super::VNODEFLAGS::FILE,
            },
        }
    }
}
use core::cell::RefCell;
impl vfs::vnode for tmpfsfile {
    fn create(&mut self, name: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn lookup(&self, child: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn mkdir(&mut self, name: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn read(&self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, UNIXERROR> {
        let mut size = count;
        if buf.len() == 0 {
            return Err(UNIXERROR::EINVAL);
        }
        if (offset + count) > self.data.len() {
            size = self.data.len();
        }
        if self.data.len() == 0 {
            return Err(UNIXERROR::EPERM);
        }
        let pro = &self.data[offset..size];
        buf[..pro.len()].copy_from_slice(pro);
        return Ok(size);
    }
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, UNIXERROR> {
        let size = count;
        if buf.len() == 0 {
            return Err(UNIXERROR::EINVAL);
        }
        if offset + buf.len() > self.data.len() as usize {
            self.data.resize((offset + buf.len()), 0);
            self.attrib.size = self.data.len();
        }
        if self.data.len() == 0 {
            return Err(UNIXERROR::EPERM);
        }
        let pro = &buf[offset..size];
        self.data.copy_from_slice(pro);

        // SO EASY
        return Ok(size);
    }
    fn get_attrib(&self) -> Result<&vfs::vnodeattributetable, UNIXERROR> {
        Ok(&self.attrib)
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
use alloc::string::ToString;
impl vfs::vnode for tmpfsdir {
    fn create(&mut self, name: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        let dir = &mut self.files;
        let ifeelsick = Arc::new(Mutex::new(tmpfsfile::default()));
        dir.insert(name.to_string(), ifeelsick.clone());
        return Ok(ifeelsick);
    }
    fn lookup(&self, child: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        if self.files.contains_key(child) {
            if let Some(qt) = self.files.get(child) {
                return Ok(qt.clone());
            }
        }
        return Err(UNIXERROR::EISDIR);
    }
    fn mkdir(&mut self, name: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        let dir = &mut self.files;
        let ifeelsick = Arc::new(Mutex::new(tmpfsdir::default()));
        dir.insert(String::from(name), ifeelsick.clone());
        return Ok(ifeelsick);
    }
    fn read(&self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
    fn get_attrib(&self) -> Result<&vnodeattributetable, UNIXERROR> {
        Ok(&self.attrib)
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}