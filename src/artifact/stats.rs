use std::fmt;

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Stats {
    HP,           // HP
    HP_,          // HP%
    Atk,          // ATK
    Atk_,         // ATK%
    Def,          // DEF
    Def_,         // DEF%
    EleMas,       // Elemental Mastery
    EnerRech_,    // Energy Recharge%
    Heal_,        // Healing Bonus%
    CritRate_,    // CRIT Rate%
    CritDMG_,     // CRIT DMG%
    PhysicalDmg_, // Physical DMG Bonus%
    AnemoDmg_,    // Anemo DMG Bonus%
    GeoDmg_,      // Geo DMG Bonus%
    ElectroDmg_,  // Electro DMG Bonus%
    HydroDmg_,    // Hydro DMG Bonus%
    PyroDmg_,     // Pyro DMG Bonus%
    CryoDmg_,     // Cryo DMG Bonus%
}

impl Stats {
    pub fn list() -> Vec<Stats> {
        vec![
            Stats::HP,           // HP
            Stats::HP_,          // HP%
            Stats::Atk,          // ATK
            Stats::Atk_,         // ATK%
            Stats::Def,          // DEF
            Stats::Def_,         // DEF%
            Stats::EleMas,       // Elemental Mastery
            Stats::EnerRech_,    // Energy Recharge%
            Stats::Heal_,        // Healing Bonus%
            Stats::CritRate_,    // CRIT Rate%
            Stats::CritDMG_,     // CRIT DMG%
            Stats::PhysicalDmg_, // Physical DMG Bonus%
            Stats::AnemoDmg_,    // Anemo DMG Bonus%
            Stats::GeoDmg_,      // Geo DMG Bonus%
            Stats::ElectroDmg_,  // Electro DMG Bonus%
            Stats::HydroDmg_,    // Hydro DMG Bonus%
            Stats::PyroDmg_,     // Pyro DMG Bonus%
            Stats::CryoDmg_,     // Cryo DMG Bonus%
        ]
    }
    pub fn get_sub_stat_weight(&self) -> f64 {
        match self {
            Stats::HP => 150.0,
            Stats::HP_ => 100.0,
            Stats::Atk => 150.0,
            Stats::Atk_ => 100.0,
            Stats::Def => 150.0,
            Stats::Def_ => 100.0,
            Stats::EleMas => 100.0,
            Stats::EnerRech_ => 100.0,
            Stats::CritRate_ => 75.0,
            Stats::CritDMG_ => 75.0,
            _ => 0.0,
        }
    }
    pub fn to_string(&self) -> &'static str {
        match self {
            Stats::HP => "HP",
            Stats::HP_ => "HP_",
            Stats::Atk => "Atk",
            Stats::Atk_ => "Atk_",
            Stats::Def => "Def",
            Stats::Def_ => "Def_",
            Stats::EleMas => "EleMas",
            Stats::EnerRech_ => "EnerRech_",
            Stats::Heal_ => "Heal_",
            Stats::CritRate_ => "CritRate_",
            Stats::CritDMG_ => "CritDMG_",
            Stats::PhysicalDmg_ => "PhysicalDmg_",
            Stats::AnemoDmg_ => "AnemoDmg_",
            Stats::GeoDmg_ => "GeoDmg_",
            Stats::ElectroDmg_ => "ElectroDmg_",
            Stats::HydroDmg_ => "HydroDmg_",
            Stats::PyroDmg_ => "PyroDmg_",
            Stats::CryoDmg_ => "CryoDmg_",
        }
    }
    pub fn can_be_sub_stat(&self) -> bool {
        matches!(
            self,
            Stats::HP
                | Stats::HP_
                | Stats::Atk
                | Stats::Atk_
                | Stats::Def
                | Stats::Def_
                | Stats::EleMas
                | Stats::EnerRech_
                | Stats::CritRate_
                | Stats::CritDMG_
        )
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Stats::HP => "?????????",
            Stats::HP_ => "??????????????????",
            Stats::Atk => "?????????",
            Stats::Atk_ => "??????????????????",
            Stats::Def => "?????????",
            Stats::Def_ => "??????????????????",
            Stats::EleMas => "????????????",
            Stats::EnerRech_ => "??????????????????",
            Stats::Heal_ => "????????????",
            Stats::CritRate_ => "?????????",
            Stats::CritDMG_ => "????????????",
            Stats::PhysicalDmg_ => "??????????????????",
            Stats::AnemoDmg_ => "?????????????????????",
            Stats::GeoDmg_ => "?????????????????????",
            Stats::ElectroDmg_ => "?????????????????????",
            Stats::HydroDmg_ => "?????????????????????",
            Stats::PyroDmg_ => "?????????????????????",
            Stats::CryoDmg_ => "?????????????????????",
        };
        write!(f, "{}", str)
    }
}
