extern crate ignore;
extern crate tre;

use ignore::Error;
use tre::config::{parse_args, Options};
use tre::display;
use tre::walker;

fn main() {
    stream_tree(&parse_args()).expect("Something went wrong...")
}

fn stream_tree(opt: &Options) -> Result<(), Error> {
    let mut prev_depth: usize = 0;
    let mut walker = walker::build(&opt)?;
    let mut parents = vec![];

    if let Some(Ok(mut prev)) = walker.next() {
        for dir in walker {
            if let Ok(curr) = dir {
                if opt.dir_only && !curr.file_type().map(|f| f.is_dir()).unwrap_or(false) {
                    continue;
                }
                let curr_depth = curr.depth();
                let mut is_last = false;
                if prev_depth != curr_depth {
                    if prev_depth < curr_depth {
                        parents.push(curr_depth)
                    } else {
                        parents.pop();
                    }
                    prev_depth = curr_depth;
                    is_last = true;
                }
                display::print(&prev, is_last, &parents);
                prev = curr;
            }
        }
        display::print(&prev, true, &parents);
    }
    Ok(())
}
