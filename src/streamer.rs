use config::Options;
use display;
use ignore::{DirEntry, Error, Walk};
use std::path::PathBuf;
use walker::build_shallow;

pub fn stream_tree(opts: &Options, walker: &mut Walk, parent_vec: Vec<bool>) -> Result<(), Error> {
    let mut dir_iter = walker.into_iter().filter_map(|e| e.ok()).peekable();

    while let Some(dir) = dir_iter.next() {
        match dir_iter.peek() {
            Some(_) => switch_tree(&opts, dir, false, parent_vec.clone()),
            None => switch_tree(&opts, dir, true, parent_vec.clone()),
        }
    }

    Ok(())
}

fn switch_tree(opts: &Options, entry: DirEntry, is_last: bool, mut parent_vec: Vec<bool>) {
    match entry.file_type() {
        // If the file is a directory
        Some(f_type) if f_type.is_dir() => {
            display::print_node(&entry, is_last, parent_vec.clone());

            let root = opts.root.clone();

            if root == PathBuf::from(entry.path()) {
                return;
            }

            parent_vec.push(!is_last);

            let new_path = entry.path();

            let new_options = Options {
                root: PathBuf::from(entry.path()),
                max_depth: opts.max_depth,
                follow_sym_links: opts.follow_sym_links,
                show_hidden: opts.show_hidden,
                dir_only: opts.dir_only,
                pattern: opts.pattern.clone(),
                extension: opts.extension.clone(),
                line_count: opts.line_count,
                no_colours: opts.no_colours,
            };

            let mut new_walk = build_shallow(&new_path, &new_options).unwrap();
            let _ = stream_tree(&new_options, &mut new_walk, parent_vec.clone());
        }

        Some(f_type) if f_type.is_file() => {
            display::print_node(&entry, is_last, parent_vec.clone());
        }

        Some(f_type) if f_type.is_symlink() => {
            display::print_node(&entry, is_last, parent_vec.clone());
        }

        Some(_) => panic!("Invalid file type"),
        None => panic!("Not a valid file"),
    };

    if is_last {
        parent_vec.pop();
    }
}
