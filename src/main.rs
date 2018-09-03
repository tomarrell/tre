extern crate tre;

use std::path::PathBuf;

use tre::node::*;

fn main() {
    let path = PathBuf::from(".");
    let root = get_nodes(path, Options::default());
    root.print();
}
