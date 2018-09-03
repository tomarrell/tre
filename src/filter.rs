use node::{Node, Options};

pub struct Actions {
    print: bool,
    dive: bool,
}

pub fn validate_node(node: Node, options: Options) -> Actions {
    if !options.show_hidden && is_node_hidden(&node) {
        return Actions {
            print: false,
            dive: false,
        };
    };

    Actions {
        print: true,
        dive: true,
    }
}

fn is_node_hidden(node: &Node) -> bool {
    let os_str = match node.path.as_path().file_name() {
        Some(os_str) => os_str,
        None => return false,
    };

    let file_name = match os_str.to_str() {
        Some(name) => name,
        None => return false,
    };

    file_name.starts_with(".")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn it_works() {
        assert_eq!("Hello", "Hello");
    }

    #[test]
    fn validate_path_works() {
        let node = Node {
            depth: 2,
            path: PathBuf::from("/some/file.rs"),
            is_last: false,
            children: vec![],
        };
        let result = validate_node(node, Options::default());
        assert!(result.print);
        assert!(result.dive);
    }

    #[test]
    fn validate_path_works_hidden_file() {
        let node = Node {
            depth: 2,
            path: PathBuf::from("/some/.file.rs"),
            is_last: false,
            children: vec![],
        };
        let result = validate_node(node, Options::default());
        assert!(!result.print);
        assert!(!result.dive);
    }

    #[test]
    fn is_node_hidden_works_with_hidden() {
        let node = Node {
            depth: 2,
            path: PathBuf::from("/some/.file.rs"),
            is_last: false,
            children: vec![],
        };
        assert!(is_node_hidden(&node));
    }

    #[test]
    fn is_node_hidden_works_with_non_hidden() {
        let node = Node {
            depth: 2,
            path: PathBuf::from("/some/file.rs"),
            is_last: false,
            children: vec![],
        };
        assert!(!is_node_hidden(&node));
    }
}
