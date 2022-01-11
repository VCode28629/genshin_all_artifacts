mod slot;
mod stats;

pub use slot::*;
pub use stats::*;
use std::{collections::HashMap, fmt};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Artifact {
    slot: Slot,
    main_stat: Stats,
    sub_stats: [Option<(Stats, u8)>; 4],
}

impl Artifact {
    pub fn get_slot(&self) -> &Slot {
        &self.slot
    }
    pub fn get_main_stat(&self) -> &Stats {
        &self.main_stat
    }
    pub fn get_sub_stats(&self) -> &[Option<(Stats, u8)>] {
        &self.sub_stats
    }
    pub fn new(slot: Slot, main_stat: Stats) -> Artifact {
        Artifact {
            slot,
            main_stat,
            sub_stats: [None, None, None, None],
        }
    }
    fn have_stat(&self, stat: &Stats) -> bool {
        if *stat == self.main_stat {
            return true;
        }
        for j in 0..4 {
            if let Some((used, _)) = &self.sub_stats[j] {
                if used == stat {
                    return true;
                }
            }
        }
        false
    }
    fn get_sub_stat_probability(&self, stat: &Stats) -> f64 {
        if self.have_stat(stat) {
            return 0.0;
        }
        let mut sum = 0.0;
        for i in Stats::list() {
            if self.have_stat(&i) {
                continue;
            }
            sum += i.get_sub_stat_weight();
        }
        stat.get_sub_stat_weight() / sum
    }
    pub fn gain(&self) -> Vec<ArtifactP> {
        let mut ret = Vec::new();
        if self.sub_stats[3].is_some() {
            // 四个词条已经满了
            // 枚举强化词条 i
            for i in 0..4 {
                let mut arti = self.clone();
                if let Some((_, cnt)) = &mut arti.sub_stats[i] {
                    *cnt += 1;
                }
                ret.push(ArtifactP(arti, 0.25));
            }
        } else {
            // 四个词条未满
            // 先找到第一个空词条
            let mut empty_index = 0;
            while self.sub_stats[empty_index].is_some() {
                empty_index += 1;
            }
            // 枚举新词条
            for stat in Stats::list() {
                // 获取当前圣遗物强化出本词条的概率
                // 已考虑词条重复情况
                let p = self.get_sub_stat_probability(&stat);
                if p == 0.0 {
                    continue;
                }
                let mut arti = self.clone();
                arti.sub_stats[empty_index] = Some((stat, 1));
                ret.push(ArtifactP(arti, p));
            }
        }
        ret
    }
    pub fn easy(&mut self) {
        let mut p = 0;
        for finding_stat in Stats::list() {
            for (stat_location, found_stat) in self.sub_stats.iter().enumerate() {
                if let Some((stat, _)) = found_stat {
                    if finding_stat == *stat {
                        self.sub_stats.swap(p, stat_location);
                        p += 1;
                        break;
                    }
                }
            }
        }
    }
}

impl fmt::Display for Artifact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = format!("{}\t{}", self.slot, self.main_stat);
        let mut map = HashMap::new();
        for i in 0..4 {
            if let Some((a, b)) = &self.sub_stats[i] {
                map.insert(a, *b);
            }
        }
        for i in Stats::list() {
            res = if i.can_be_sub_stat() {
                format!(
                    "{}\t{}",
                    res,
                    match map.get(&i) {
                        Some(x) => *x,
                        None => 0,
                    }
                )
            } else {
                res
            }
        }
        write!(f, "{}", res)
    }
}

#[derive(Clone)]
pub struct ArtifactP(pub Artifact, pub f64);

impl ArtifactP {
    pub fn gain(&self) -> Vec<ArtifactP> {
        let mut ret = self.0.gain();
        for i in ret.iter_mut() {
            i.1 *= self.1;
        }
        ret
    }
}
