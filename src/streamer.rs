use config::Options;
use display;
use ignore::{DirEntry, Error, Walk};
use stats_collector::{FileType, StatsCollector};

pub struct Streamer {
    prev_depth: usize,
    curr_depth: usize,
    parent_depths: Vec<usize>,
    curr_line_count: Option<usize>,
    collector: StatsCollector,
    walker: Walk,
    dir_only: bool,
    count_lines: bool,
}

impl Streamer {
    pub fn new(opt: Options, collector: StatsCollector, walker: Walk) -> Streamer {
        Streamer {
            prev_depth: 0,
            curr_depth: 0,
            parent_depths: vec![],
            curr_line_count: None,
            collector: collector,
            walker: walker,
            dir_only: opt.dir_only,
            count_lines: opt.line_count,
        }
    }

    pub fn stream_tree(&mut self) -> Result<(), Error> {
        let mut prev = self
            .walker
            .next()
            .expect("could not get first element from walker")
            .expect("could not get first element from walker");

        // walker traverses depth first
        for dir in self.walker {
            match dir {
                Ok(curr) => {
                    self.stream_node(&prev);
                    prev = curr;
                }
                // TODO currently just ignore the dir if can't parse it
                _ => continue,
            }
        }

        display::print(&prev, true, &self.parent_depths, self.curr_line_count);

        self.collector.print_stats();
        Ok(())
    }

    fn stream_node(&mut self, node: &DirEntry) -> Result<(), Error> {
        let mut is_last = false;

        //parses current file type and tally stats
        let file_type = self.collector.parse_and_collect(node)?;

        // match on file type to do additional logic, such as skip directory or do line counting
        match file_type {
            FileType::File if self.dir_only => return Ok(()),
            FileType::File if self.count_lines => {
                self.curr_line_count = Some(self.collector.count_lines(node)?);
            }
            _ => self.curr_line_count = None,
        }

        // This logic allows us to keep record parents (store in vec) of the current file.
        // We are always traversing one file ahead of what we print, so we can tell whether the thing to print
        // is the last of is directory (when the depth changes)
        if self.prev_depth != self.curr_depth {
            if self.prev_depth < self.curr_depth && self.curr_depth > 1 {
                self.parent_depths.push(self.curr_depth)
            } else {
                self.parent_depths.pop();
            }

            self.prev_depth = self.curr_depth;
            is_last = true;
        }

        display::print(node, is_last, &self.parent_depths, self.curr_line_count);
        Ok(())
    }
}