extern crate alloc;
use hashbrown::HashMap;
use alloc::string::String;

use super::vfs::{vnode, vnodeattributetable};
use alloc::sync::Arc;
use alloc::boxed::Box;
use spin::Mutex;
use crate::fs::tmpfs::tmpfsdir;
use crate::fs::vfs;
pub mod null;
pub struct devfsdir {
    files: HashMap<String, Arc<Mutex<dyn vfs::vnode>>>,
    attrib: vnodeattributetable,
}
pub struct devfsfile {
    pub ops: Option<Box<dyn devfsops>>,
    attrib: vnodeattributetable
}
impl Default for devfsdir {
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
impl<'a> Default for devfsfile {
    fn default() -> Self {
        Self {
            ops: None,
            attrib: vnodeattributetable {
                size: 0,
                TYPE: super::VNODEFLAGS::FILE,
            },
        }
    }
}
pub trait devfsops {
    fn read(&self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, UNIXERROR>;
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, crate::utils::UNIXERROR>;
}
trait devfsnode {
    fn dev_create(&mut self, name: &str, ops: Box<dyn devfsops>) -> Result<Arc<Mutex<dyn vnode>>, UNIXERROR>;
}
use alloc::string::ToString;
use crate::utils::UNIXERROR;
impl vnode for devfsdir {
    fn create(&mut self, name: &str) -> Result<Arc<Mutex<dyn vnode>>, crate::utils::UNIXERROR> {
        let dir = &mut self.files;
        let ifeelsick = Arc::new(Mutex::new(devfsfile::default()));
        dir.insert(name.to_string(), ifeelsick.clone());
        return Ok(ifeelsick);
    }
    fn get_attrib(&self) -> Result<&vnodeattributetable, crate::utils::UNIXERROR> {
        Ok(&self.attrib)
    }
    fn lookup(&self, child: &str) -> Result<Arc<Mutex<dyn vnode>>, crate::utils::UNIXERROR> {
        if self.files.contains_key(child) {
            if let Some(qt) = self.files.get(child) {
                return Ok(qt.clone());
            }
        }
        return Err(UNIXERROR::EISDIR);
    }
    fn mkdir(&mut self, name: &str) -> Result<Arc<Mutex<dyn vnode>>, crate::utils::UNIXERROR> {
        let dir = &mut self.files;
        let ifeelsick = Arc::new(Mutex::new(devfsdir::default()));
        dir.insert(String::from(name), ifeelsick.clone());
        return Ok(ifeelsick);
    }
    fn read(&self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, crate::utils::UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, crate::utils::UNIXERROR> {
        return Err(UNIXERROR::EISDIR);
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}
impl vnode for devfsfile {
    fn create(&mut self, name: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn lookup(&self, child: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn mkdir(&mut self, name: &str) -> Result<Arc<Mutex<dyn vfs::vnode>>, UNIXERROR> {
        return Err(UNIXERROR::EISFILE);
    }
    fn get_attrib(&self) -> Result<&vnodeattributetable, UNIXERROR> {
        Ok(&self.attrib)
    }
    fn read(&self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, UNIXERROR> {
        if self.ops.is_none() {
            return Err(UNIXERROR::ENOENT);
        }
        let res = self.ops.as_ref().unwrap().read(buf, offset, count);
        return res;
    }
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, UNIXERROR> {
        if self.ops.is_none() {
            return Err(UNIXERROR::ENOENT);
        }
        let res = self.ops.as_mut().unwrap().write(buf, offset, count);
        return res;
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
    
}

impl devfsnode for devfsdir {
    fn dev_create(&mut self, name: &str, ops: Box<dyn devfsops>) -> Result<Arc<Mutex<dyn vnode>>, UNIXERROR>{
        // WE HAVE NO CHOICE AND QWINCI AINT AWAKE
        // TODO: FIX LATER
        let mut o = self.create(name);
        o.as_mut().unwrap().lock().as_any_mut().downcast_mut::<devfsfile>().unwrap().ops = Some(ops);
        return o;
    }
}
#[test]
fn devfs_test() {
    let mut new: devfsdir = devfsdir::default();
    let x = new.dev_create("null", Box::new(null::NullDriver)).unwrap();
    let mut buf = [0;256];
    let j = x.lock().as_any_mut().downcast_mut::<devfsfile>().unwrap().read(&mut buf, 0, 5);
    let oj = j.unwrap();
    assert_eq!(oj, 0);
    let ooo = x.lock().as_any_mut().downcast_mut::<devfsfile>().unwrap().write(&buf, 0, 500);
    assert_eq!(ooo.unwrap(), 500)
}
use vfs::CUR_VFS;
use crate::println;
pub fn devfs_init() {
    let construct = devfsdir::default();
    let s;
    unsafe {
        s = CUR_VFS.as_mut().unwrap().vnode.as_mut().unwrap();
    };
        if s.lock().lookup("dev").is_err() {
            println!("Creating DevFS");
            s.lock().as_any_mut().downcast_mut::<tmpfsdir>().unwrap().files.insert("dev".to_string(), Arc::new(Mutex::new(devfsdir::default())));
        }
    let fs = s.lock().lookup(
        "dev"
    ).unwrap();
    fs.lock().as_any_mut().downcast_mut::<devfsdir>().unwrap().dev_create("null", Box::new(null::NullDriver)).unwrap();
    
}