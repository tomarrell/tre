use clap::{App, Arg};

pub struct Options {
    pub max_depth: Option<usize>,
    pub follow_sym_links: bool,
    pub show_hidden: bool,
    pub root: Option<String>,
    pub dir_only: bool,
    pub pattern: Option<String>,
    pub extension: Option<String>,
}

impl Options {
    pub fn new(
        root: Option<String>,
        max_depth: Option<usize>,
        follow_sym_links: bool,
        show_hidden: bool,
        dir_only: bool,
        pattern: Option<String>,
        extension: Option<String>,
    ) -> Options {
        Options {
            root,
            max_depth,
            follow_sym_links,
            show_hidden,
            dir_only,
            pattern,
            extension,
        }
    }
}

pub fn parse_args() -> Options {
    let matches = App::new("tre")
        .version("1.0")
        .author("Jacky Zhen. <me@jackyzhen.com>, Tom Arrell. <me@tom.arrell.com>")
        .about("List contents of directories in a tree-like format.")
        .arg(
            Arg::with_name("path")
                .help("Start from specific path.")
                .index(1)
                .takes_value(true),
        ).arg(
            Arg::with_name("directories")
                .short("d")
                .long("directories")
                .help("List directories only.")
                .takes_value(false),
        ).arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .help("All files are printed. By default tre  does  not  print  hidden files  (those  beginning  with a dot `.').  In no event does tree print the file system constructs `.' (current directory) and `..' (previous directory).")
                .takes_value(false),
        ).arg(
            Arg::with_name("symbolic")
                .short("s")
                .long("symbolic")
                .help("Follows symbolic links if they point to directories, as if they were directories")
                .takes_value(false),
        ).arg(
            Arg::with_name("level")
                .short("l")
                .long("level")
                .help("Max display depth of the directory tree.")
                .validator(is_numeric)
                .takes_value(true),
        ).arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .help("filter with a name pattern.")
                .takes_value(true),
        ).arg(
            Arg::with_name("extension")
                .short("e")
                .long("extension")
                .help("filter with a file extension.")
                .takes_value(true),
        ).get_matches();

    let max_depth: Option<usize> = match matches.value_of("level") {
        Some(l) => Some(l.parse().unwrap()),
        None => None,
    };

    Options::new(
        matches.value_of("path").map(|s| s.to_string()),
        max_depth,
        matches.is_present("symbolic"),
        matches.is_present("all"),
        matches.is_present("directories"),
        matches.value_of("pattern").map(|s| s.to_string()),
        matches.value_of("extension").map(|s| s.to_string()),
    )
}

fn is_numeric(val: String) -> Result<(), String> {
    match val.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Value wasn't a valid number!")),
    }
}
