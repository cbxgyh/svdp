use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use ::serde_json;
use serde_json::Value;

use super::{DT, META, DATA, Ecs};

pub mod rd;
pub mod wr;
pub mod rd_tps;
pub mod wr_tps;

pub struct Disk {
    disk_rate: i32,

    rd: u32,  /* kbytes */
    wr: u32,
    rdio: u32,  /* tps */
    wrio: u32,
}

pub struct Meta();
pub struct Data();  /* disk rate */

impl META for Meta {
    fn argv_new(&self, region: String) -> Vec<String> {
        vec![
            "-region".to_owned(),
            region.to_owned(),
            "-domain".to_owned(),
            "ecs.aliyuncs.com".to_owned(),
            "-apiName".to_owned(),
            "DescribeDisks".to_owned(),
            "-apiVersion".to_owned(),
            "2014-05-26".to_owned(),
            "Action".to_owned(),
            "DescribeDisks".to_owned(),
            "PageSize".to_owned(),
            "100".to_owned(),
        ]
    }

    fn insert(&self, holder: &Arc<Mutex<HashMap<String, Ecs>>>, data: Vec<u8>) {
        let v: Value = serde_json::from_slice(&data).unwrap_or(Value::Null);
        if Value::Null == v {
            return;
        }

        let body = &v["Disks"]["Disk"];
        let mut diskid;
        let mut device;
        for i in 0.. {
            if Value::Null == body[i] {
                break;
            } else {
                if let Value::String(ref ecsid) = body[i]["InstanceId"] {
                    if let Some(ecs) = holder.lock().unwrap().get_mut(ecsid) {
                        if let Value::String(ref id) = body[i]["DiskId"] {
                            diskid= id;
                        } else {
                            continue;
                        }

                        if let Value::String(ref dev) = body[i]["Device"] {
                            device = dev;
                        } else {
                            continue;
                        }

                        ecs.disk.insert((*device).clone(), (*diskid).clone());
                    }
                }
            }
        }
    }

    fn reflect(&self) -> DT {
        DT::Disk
    }
}

impl DATA for Data {
    fn argv_new(&self, region: String) -> Vec<String> {
        let mut argv = self.argv_new_base(region);
        argv.push("diskusage_utilization".to_owned());

        argv.push("StartTime".to_owned());
        unsafe {
            argv.push(::BASESTAMP.to_string());
        }

        argv.push("EndTime".to_owned());
        unsafe {
            argv.push((::BASESTAMP + ::INTERVAL).to_string());
        }

        argv
    }

    fn insert(&self, holder: &Arc<Mutex<HashMap<String, Ecs>>>, data: Vec<u8>) {
    }
}
