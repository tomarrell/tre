use display::print_node;
use std::fs;
use std::path::PathBuf;

pub struct Node {
    pub depth: usize,
    pub path: PathBuf,
    pub is_last: bool,
    pub children: Vec<Node>,
}

impl Node {
    pub fn print(&self) {
        println!("{}", print_node(&self));
        if !self.path.is_dir() {
            return;
        }
        for child in self.children.iter() {
            child.print();
        }
    }
}

pub struct Options {
    max_depth: Option<usize>,
    follow_sym_links: bool,
    show_hidden: bool,
}

impl Options {
    pub fn new(max_depth: Option<usize>, follow_sym_links: bool, show_hidden: bool) -> Options {
        Options {
            max_depth,
            follow_sym_links,
            show_hidden,
        }
    }

    pub fn default() -> Options {
        Options {
            max_depth: None,
            follow_sym_links: false,
            show_hidden: false,
        }
    }
}

pub fn get_nodes(root: PathBuf, options: Options) -> Node {
    let mut curr = Node {
        path: root,
        children: Vec::new(),
        depth: 0,
        is_last: false,
    };
    get_nodes_recursive(&mut curr, &options);
    curr
}

fn get_nodes_recursive(root: &mut Node, options: &Options) {
    if !root.path.is_dir() {
        return;
    }
    if let Some(max_depth) = options.max_depth {
        if root.depth == max_depth {
            return;
        }
    }
    let metadata = fs::symlink_metadata(root.path.clone()).expect("failed to fetch file meta data");
    let file_type = metadata.file_type();
    if file_type.is_symlink() && !options.follow_sym_links {
        return;
    }

    for entry in fs::read_dir(root.path.as_path()).expect("failed to read path") {
        let path = entry.unwrap().path();
        let mut curr = Node {
            path,
            children: Vec::new(),
            depth: root.depth + 1,
            is_last: false,
        };
        get_nodes_recursive(&mut curr, &options);
        root.children.push(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::get_nodes;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        let path = PathBuf::from(".");
        let nodes = get_nodes(path);
        assert_eq!("Hello", "Hello");
    }
}
