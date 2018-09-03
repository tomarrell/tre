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

pub fn get_nodes(root: PathBuf) -> Node {
    let mut curr = Node {
        path: root,
        children: Vec::new(),
        depth: 0,
        is_last: false,
    };
    get_nodes_recursive(&mut curr);
    curr
}

fn get_nodes_recursive(root: &mut Node) {
    if !root.path.is_dir() {
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
        get_nodes_recursive(&mut curr);
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
        nodes.print();
    }
}
