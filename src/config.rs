pub extern crate json;

use crate::artifact::{Slot, Stats};
use std::{fs::File, io::Read};

pub const MAIN_STAT: &str = "main_stat";
pub const SUB_STAT: &str = "sub_stat";
pub const USING: &str = "using";

pub fn get_config(path: &str) -> json::JsonValue {
    let mut file = File::open(path).expect("cannot find config.json");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("fail to read file");
    let mut ret = json::parse(&string).expect("not a json object");

    if ret[SUB_STAT][USING].is_boolean() && ret[SUB_STAT][USING] == true {
        for i in Slot::list() {
            let slot = i.to_string();
            for j in Stats::list() {
                if !j.can_be_sub_stat() {
                    continue;
                }
                let stat = j.to_string();
                if ret[SUB_STAT][stat].is_null() {
                    ret[SUB_STAT][stat] = 0.0.into();
                }
                ret[slot][SUB_STAT][stat] = ret[SUB_STAT][stat].clone();
            }
        }
    }
    for i in Slot::list() {
        let slot = i.to_string();
        for j in Stats::list() {
            let stat = j.to_string();
            if ret[slot][MAIN_STAT][stat].is_null() {
                ret[slot][MAIN_STAT][stat] = false.into();
            }
            if !ret[slot][MAIN_STAT][stat].is_boolean() {
                panic!(
                    "config.json: {}.{}.{} is not a boolean",
                    slot, MAIN_STAT, stat
                );
            }
            if !j.can_be_sub_stat() {
                continue;
            }
            if !ret[slot][SUB_STAT][stat].is_number() {
                panic!(
                    "config.json: {}.{}.{} is not a number",
                    slot, SUB_STAT, stat
                );
            }
        }
    }
    ret
}
