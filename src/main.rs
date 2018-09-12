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
    let mut files = 0;
    let mut directories = 1;
    let mut links = 0;
    let mut lines = 0;

    if let Some(Ok(mut prev)) = walker.next() {
        for dir in walker {
            if let Ok(curr) = dir {
                match curr.file_type() {
                    Some(typ) if typ.is_dir() => {
                        directories += 1;
                    }
                    Some(typ) if opt.dir_only && !typ.is_dir() => {
                        directories += 1;
                        continue;
                    }
                    Some(typ) if typ.is_symlink() => {
                        links += 1;
                    }
                    _ => {
                        files += 1;
                    }
                }

                let curr_depth = curr.depth();
                let mut is_last = false;
                if prev_depth != curr_depth {
                    if prev_depth < curr_depth && curr_depth > 1 {
                        parents.push(curr_depth)
                    } else {
                        parents.pop();
                    }
                    prev_depth = curr_depth;
                    is_last = true;
                }
                lines += display::print(&prev, is_last, &parents, opt.line_count)?;
                prev = curr;
            }
        }
        lines += display::print(&prev, true, &parents, opt.line_count)?;
    }
    display::print_stats(files, directories, links, lines, opt.line_count);
    Ok(())
}
