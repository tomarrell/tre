extern crate tre;

use tre::*;

use config;
use stats_collector::StatsCollector;
use streamer::Streamer;
use walker;

fn main() {
    let options = config::parse_args();
    let collector = StatsCollector::new();

    let mut walker = match walker::build(&options) {
        Ok(walk) => walk,
        Err(error) => panic!(
            "Failed to build directory walker with specified options.\n>>> {}",
            error
        ),
    };

    Streamer::new(options, collector)
        .stream_tree(&mut walker)
        .expect("could not stream tree.");
}
