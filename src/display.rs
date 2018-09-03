use node::Node;
use std::path::PathBuf;

pub fn print_node(node: &Node) -> String {
    let optional_path = match node.path.file_name() {
        Some(name) => name.to_str(),
        None => return String::new(),
    };

    let path_name = match optional_path {
        Some(name) => name,
        None => panic!("Failed to decode path name"),
    };

    let first_char = if node.is_last {
        "└"
    } else {
        "├"
    };

    format!(
        "{:spaces$}{}── {}",
        "",
        first_char,
        path_name,
        spaces = (node.depth * 4)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_node_works() {
        let node = Node {
            depth: 1,
            path: PathBuf::from("/test/filename.rs"),
            children: vec![],
            is_last: false,
        };

        assert_eq!("    ├── filename.rs", print_node(&node));
    }

    #[test]
    fn print_node_works_width_depth() {
        let node = Node {
            depth: 3,
            path: PathBuf::from("/test/otherfile.rs"),
            children: vec![],
            is_last: false,
        };

        assert_eq!("            ├── otherfile.rs", print_node(&node));
    }

    #[test]
    fn print_node_works_last_item_in_dir() {
        let node = Node {
            depth: 0,
            path: PathBuf::from("/test/lastfile.rs"),
            children: vec![],
            is_last: true,
        };

        assert_eq!("└── lastfile.rs", print_node(&node));
    }

    #[test]
    fn it_works() {
        assert_eq!("Hello", "Hello");
    }
}
