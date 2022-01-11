use crate::artifact::Artifact;
use crate::config::{self, json::JsonValue};

pub(crate) fn get_score(config: &JsonValue, arti: &Artifact) -> f64 {
    let slot = arti.get_slot().to_string();
    let main_stat = arti.get_main_stat().to_string();
    if !config[slot][config::MAIN_STAT][main_stat]
        .as_bool()
        .expect("wrong config format")
    {
        return 0.0;
    }
    let mut score = 0.0;
    for (stat, cnt) in arti.get_sub_stats().iter().flatten() {
        score += *cnt as f64
            * config[slot][config::SUB_STAT][stat.to_string()]
                .as_f64()
                .expect("wrong config format");
    }
    score
}
