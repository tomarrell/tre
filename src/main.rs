extern crate tre;

use tre::*;

use config;
// use stats_collector::StatsCollector;
use streamer::{stream_tree};
use walker;
use std::path::Path;

fn main() {
    let options = config::parse_args();
    // let collector = StatsCollector::new();
    let path = Path::new(&options.root);

    let mut walker = match walker::build_shallow(path, &options) {
        Ok(walk) => walk,
        Err(error) => panic!(
            "Failed to build directory walker with specified options.\n>>> {}",
            error
        ),
    };

    stream_tree(&options, &mut walker, Vec::new())
        .expect("could not stream tree.");
}
