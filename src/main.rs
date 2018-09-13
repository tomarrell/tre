extern crate tre;

use tre::*;

fn main() {
    let options = tre::config::parse_args();
    let collector = tre::stats_collector::StatsCollector::new();

    // TODO: decide how to handle errors
    let walker = walker::build(&options).expect("could not create walker.");

    tre::streamer::stream_tree(options, collector, walker).expect("could not stream tree.");
}
