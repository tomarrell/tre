use ignore::DirEntry;

pub fn print_node(entry: &DirEntry, is_last: bool, parent_vec: Vec<bool>) {
    let optional_path = match entry.path().file_name() {
        Some(name) => name.to_str(),
        None => {
            println!(".");
            return ();
        }
    };

    let path_name = match optional_path {
        Some(name) => name,
        None => panic!("Failed to decode path name"),
    };

    let first_char = if is_last { "└" } else { "├" };

    let mut prefix_string = String::new();

    for is_parent_not_last in parent_vec {
        if is_parent_not_last {
            prefix_string.push_str("│   ");
        } else {
            prefix_string.push_str("    ");
        }
    }

    println!(
        "{}",
        format!(
            "{}{}── {}",
            prefix_string,
            first_char,
            path_name,
        )
    );
}
