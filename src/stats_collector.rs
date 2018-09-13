use ignore::{DirEntry, Error};
use std::fs::File;
use std::io::prelude::*;
use std::io::Error as IOError;

pub enum FileType {
    Directory,
    File,
    Link,
}

pub struct Stats {
    files: usize,
    directories: usize,
    links: usize,
    lines: Option<usize>,
}

pub struct StatsCollector {
    stats: Stats,
}

impl StatsCollector {
    pub fn new() -> StatsCollector {
        StatsCollector {
            stats: Stats {
                files: 0,
                directories: 1,
                links: 0,
                lines: None,
            },
        }
    }
    pub fn parse_and_collect(&mut self, entry: &DirEntry) -> Result<FileType, Error> {
        match entry.file_type() {
            Some(typ) if typ.is_dir() => {
                self.stats.directories += 1;
                Ok(FileType::Directory)
            }
            Some(typ) if typ.is_symlink() => {
                self.stats.links += 1;
                Ok(FileType::Link)
            }
            _ => {
                self.stats.files += 1;
                Ok(FileType::File)
            }
        }
    }

    pub fn count_lines(&mut self, entry: &DirEntry) -> Result<usize, IOError> {
        let mut f = File::open(entry.path())?;
        let mut s = String::new();

        // TODO think of dealing with this case better
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(_) => (),
        };
        let line_count = s.lines().count();
        self.stats.lines = match self.stats.lines {
            Some(l) => Some(l + line_count),
            None => Some(line_count),
        };
        Ok(line_count)
    }

    pub fn print_stats(&self) {
        println!(
            "\n{} directories, {} files, {} symbolic links, {} lines",
            self.stats.directories,
            self.stats.files,
            self.stats.links,
            self.stats.lines.unwrap_or(0)
        );
    }
}
