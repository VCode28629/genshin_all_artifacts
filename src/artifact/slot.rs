use super::stats::Stats;
use std::fmt;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Slot {
    Flower,  // 花
    Plume,   // 羽毛
    Sands,   // 沙漏
    Goblet,  // 杯子
    Circlet, // 头
}

impl Slot {
    pub fn list() -> Vec<Slot> {
        vec![
            Slot::Flower,  // 花
            Slot::Plume,   // 羽毛
            Slot::Sands,   // 沙漏
            Slot::Goblet,  // 杯子
            Slot::Circlet, // 头
        ]
    }
    pub fn get_main_stat_probability(&self, stat: &Stats) -> f64 {
        match self {
            Slot::Flower => match stat {
                Stats::HP => 1.0,
                _ => 0.0,
            },
            Slot::Plume => match stat {
                Stats::Atk => 1.0,
                _ => 0.0,
            },
            Slot::Sands => match stat {
                Stats::HP_ => 0.2668,
                Stats::Atk_ => 0.2666,
                Stats::Def_ => 0.2666,
                Stats::EnerRech_ => 0.1,
                Stats::EleMas => 0.1,
                _ => 0.0,
            },
            Slot::Goblet => match stat {
                Stats::HP_ => 0.2125,
                Stats::Atk_ => 0.2125,
                Stats::Def_ => 0.2000,
                Stats::EleMas => 0.0250,
                Stats::PhysicalDmg_ => 0.0500,
                Stats::AnemoDmg_ => 0.0500,
                Stats::GeoDmg_ => 0.0500,
                Stats::ElectroDmg_ => 0.0500,
                Stats::HydroDmg_ => 0.0500,
                Stats::PyroDmg_ => 0.0500,
                Stats::CryoDmg_ => 0.0500,
                _ => 0.0,
            },
            Slot::Circlet => match stat {
                Stats::HP_ => 0.2200,
                Stats::Atk_ => 0.2200,
                Stats::Def_ => 0.2200,
                Stats::EleMas => 0.0400,
                Stats::Heal_ => 0.1000,
                Stats::CritRate_ => 0.1000,
                Stats::CritDMG_ => 0.1000,
                _ => 0.0,
            },
        }
    }
    pub fn to_string(&self) -> &'static str {
        match self {
            Slot::Flower => "Flower",
            Slot::Plume => "Plume",
            Slot::Sands => "Sands",
            Slot::Goblet => "Goblet",
            Slot::Circlet => "Circlet",
        }
    }
}

impl fmt::Display for Slot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Slot::Flower => "生之花",
            Slot::Plume => "死之羽",
            Slot::Sands => "时之沙",
            Slot::Goblet => "空之杯",
            Slot::Circlet => "理之冠",
        };
        write!(f, "{}", str)
    }
}
