use ignore::DirEntry;

const PARENT_CHAR: &str = "│";
const LAST_CHAR: &str = "└";
const LINE_CHAR: &str = "├";

pub fn dir_str(dir: &DirEntry, is_last: bool, parents: &Vec<usize>) -> String {
    let path_name = dir.file_name().to_owned().into_string().unwrap();
    let depth = dir.depth();

    let first_char = if is_last { LAST_CHAR } else { LINE_CHAR };
    if depth == 0 {
        path_name
    } else {
        let mut indent = String::new();
        for i in 1..depth {
            indent = format!(
                "{}{}{:space$}",
                "",
                indent,
                if parents.contains(&i) || i == 1 {
                    PARENT_CHAR
                } else {
                    " "
                },
                space = 4
            );
        }
        format!("{}{}── {}", indent, first_char, path_name)
    }
}

pub fn print(dir: &DirEntry, is_last: bool, parents: &Vec<usize>) {
    println!("{}", dir_str(dir, is_last, parents));
}

pub fn print_stats(files: usize, directories: usize, links: usize) {
    println!(
        "\n{} directories, {} files, {} symbolic links",
        directories, files, links
    );
}
