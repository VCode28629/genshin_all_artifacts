mod artifact;
mod config;
mod create_artifacts;
mod score;

use artifact::Stats;
use create_artifacts::create;
use score::get_score;

fn main() {
    let config = config::get_config("config.json");
    let artifacts = create(5);
    print!("位置\t主词条");
    for i in Stats::list() {
        if i.can_be_sub_stat() {
            print!("\t{}", i);
        }
    }
    println!("\t概率\t分数");
    for i in artifacts.iter() {
        let score = get_score(&config, &i.0);
        println!("{}\t{}\t{}", i.0, i.1, score);
    }
}
