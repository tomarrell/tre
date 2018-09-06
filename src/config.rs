use clap::{App, Arg};

pub struct Options {
    pub max_depth: Option<usize>,
    pub follow_sym_links: bool,
    pub show_hidden: bool,
    pub root: Option<String>,
}

impl Options {
    pub fn new(
        root: Option<String>,
        max_depth: Option<usize>,
        follow_sym_links: bool,
        show_hidden: bool,
    ) -> Options {
        Options {
            root,
            max_depth,
            follow_sym_links,
            show_hidden,
        }
    }
}

pub fn parse_args() -> Options {
    let matches = App::new("tre")
        .version("1.0")
        .author("Jacky Zhen. <me@jackyzhen.com>, Tom Arrell. <me@tom.arrell.com>")
        .about("List contents of directories in a tree-like format.")
        .arg(
            Arg::with_name("directory")
                .help("Target a specific directory.")
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
            Arg::with_name("full")
                .short("f")
                .long("full")
                .help("Prints the full path prefix for each file.")
                .takes_value(false),
        ).arg(
            Arg::with_name("links")
                .short("l")
                .long("links")
                .help("Follows symbolic links if they point to directories, as if they were directories")
                .takes_value(false),
        ).arg(
            Arg::with_name("level")
                .short("L")
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
        ).get_matches();

    let max_depth: Option<usize> = match matches.value_of("level") {
        Some(l) => Some(l.parse().unwrap()),
        None => None,
    };

    Options::new(
        matches.value_of("directory").map(|s| s.to_string()),
        max_depth,
        matches.is_present("links"),
        matches.is_present("all"),
    )
}

fn is_numeric(val: String) -> Result<(), String> {
    match val.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Value wasn't a valid number!")),
    }
}
