use crate::artifact::{Artifact, ArtifactP, Slot, Stats};
use std::collections::HashMap;

fn merged(list: &[ArtifactP]) -> Vec<ArtifactP> {
    let mut map = HashMap::new();
    for i in list.iter() {
        let mut arti = i.0.clone();
        arti.easy();
        let x = map.entry(arti).or_insert(0.0);
        *x += i.1;
    }
    let mut res: Vec<ArtifactP> = Vec::new();
    for i in map {
        res.push(ArtifactP(i.0, i.1)); // TODO
    }
    res
}

pub fn create(strengthen_times: u8) -> Vec<ArtifactP> {
    let mut arti_list: Vec<ArtifactP> = Vec::new();

    for slot in Slot::list() {
        for main_stat in Stats::list() {
            let p = slot.get_main_stat_probability(&main_stat);
            if p == 0.0 {
                continue;
            }
            let arti = Artifact::new(slot.clone(), main_stat); // 为什么这里不加clone也能行？
            arti_list.push(ArtifactP(arti, p));
        }
    }

    let gained_list = |arti_list: &Vec<ArtifactP>| {
        let mut ret: Vec<ArtifactP> = Vec::new();
        for i in arti_list {
            ret.append(&mut i.gain());
        }
        merged(&ret)
    };

    let arti_list = gained_list(&arti_list);
    let arti_list = gained_list(&arti_list);
    let arti_list = gained_list(&arti_list);
    let mut res = arti_list.clone();
    for i in res.iter_mut() {
        i.1 *= 0.8;
    }
    let arti_list = gained_list(&arti_list);
    for i in arti_list.iter() {
        res.push(ArtifactP(i.0.clone(), i.1 * 0.2)); // ? 这里不加clone行不行
    }
    for _ in 0..strengthen_times {
        res = gained_list(&res);
    }
    res
}
