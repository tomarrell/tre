use config::Options;
use display;
use ignore::{DirEntry, Error, Walk};
use stats_collector::{FileType, StatsCollector};
use walker::build_shallow;

/// Streamer represents the object traversing the filesystem, printing the structure and collecting stats.
pub struct Streamer {
    prev_depth: usize,
    curr_depth: usize,
    parent_depths: Vec<usize>,
    curr_line_count: Option<usize>,
    collector: StatsCollector,
    dir_only: bool,
    count_lines: bool,
    colours: bool,
    options: Options,
}

impl Streamer {
    pub fn new(opt: Options, collector: StatsCollector) -> Streamer {
        Streamer {
            prev_depth: 0,
            curr_depth: 0,
            parent_depths: vec![],
            curr_line_count: None,
            collector: collector,
            dir_only: opt.dir_only,
            count_lines: opt.line_count,
            colours: !opt.no_colours,
            options: opt,
        }
    }

    /// kicks off a recursive streaming of a directory file structure
    pub fn stream_tree(&mut self, walker: &mut Walk) -> Result<(), Error> {
        let mut prev = walker
            .next()
            .expect("could not get first element from walker")
            .expect("could not get first element from walker");
        self.prev_depth = prev.depth();

        // walker traverses depth first, ignoring files/folders it can't parse or does not have permission to
        for dir in walker.into_iter().filter_map(|e| e.ok()) {
            self.curr_depth = dir.depth();
            self.stream_node(&prev, false)?;
            prev = dir;
            self.prev_depth = self.curr_depth;
        }

        self.stream_node(&prev, true)?;

        println!("{}", self.collector);
        Ok(())
    }

    /// parse and stream an individual node, correctly printing its representation and updating statistics.
    // In need of optimization
    fn stream_node(&mut self, node: &DirEntry, is_last: bool) -> Result<(), Error> {
        let mut is_last = is_last;
        let file_name = node.file_name().to_owned().into_string().unwrap();

        //parses current file type and tally stats
        let file_type = self.collector.parse_and_collect(node)?;

        // match on file type to do additional logic, such as skip directory or do line counting
        match file_type {
            FileType::File if self.dir_only => return Ok(()),
            FileType::File if self.count_lines => {
                // if failed to count lines (.e.g. no permission to read file) just pretend its 0
                self.curr_line_count = Some(self.collector.count_lines(node).unwrap_or(0));
            }
            _ => self.curr_line_count = None,
        }

        let mut should_pop = false;
        // This logic allows us to keep record parents (store in vec) of the current file.
        // We are always traversing one file ahead of what we print, so we can tell whether the thing to print
        // is the last of its parent directory (when the depth changes)
        // additional we build a shallow walker that tries to figure out if the paren dir is the last dir,
        // if so we pop it from the vec stack because we don't need to print that branch afterwards.
        if self.prev_depth != self.curr_depth {
            if self.prev_depth < self.curr_depth {
                if let Some(parent_path) = node.path().parent() {
                    let mut shallow_walker = build_shallow(parent_path, &self.options)?
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .skip_while(|n| n.path() != node.path())
                        .skip(1);

                    if let Some(_) = shallow_walker.next() {
                        self.parent_depths.push(self.prev_depth);
                    } else {
                        is_last = true;
                    }
                }
            } else {
                should_pop = true;
                is_last = true;
            }
        }

        display::print(
            file_name,
            file_type,
            self.prev_depth,
            is_last,
            &self.parent_depths,
            self.curr_line_count,
            self.colours,
        );
        if should_pop {
            self.parent_depths.pop();
        }
        Ok(())
    }
}
