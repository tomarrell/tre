extern crate tre;

use tre::*;

fn main() {
    let options = tre::config::parse_args();
    let collector = tre::stats_collector::StatsCollector::new();

    // TODO: decide how to handle errors
    let mut walker = walker::build(&options).expect("could not create walker.");

    streamer::Streamer::new(options, collector)
        .stream_tree(&mut walker)
        .expect("could not stream tree.");
}
