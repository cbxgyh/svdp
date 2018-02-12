use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use super::Disk;
use super::super::{DATA, Ecs};
use super::super::super::{BASESTAMP, INTERVAL};

pub struct Data ();

impl DATA for Data {
    fn argv_new(&self, region: String) -> Vec<String> {
        let mut argv = self.argv_new_base(region);
        argv.push("disk_readbytes".to_owned());

        argv.push("StartTime".to_owned());
        unsafe {
            argv.push(BASESTAMP.to_string());
        }

        argv.push("EndTime".to_owned());
        unsafe {
            argv.push((BASESTAMP + INTERVAL).to_string());
        }

        argv
    }

    fn insert(&self, holder: &Arc<Mutex<HashMap<String, Ecs>>>, data: Vec<u8>) {
        let setter = |disk: &mut Disk, v: i32| disk.rd = v / 1024;

        super::insert(holder, data, setter);
    }
}
