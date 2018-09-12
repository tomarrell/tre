use ignore::DirEntry;

// const PARENT_CHAR: &str = "│";
const LAST_CHAR: &str = "└";
const LINE_CHAR: &str = "├";

pub fn dir_str(dir: &DirEntry, is_last: bool, parents: &[usize]) -> String {
    let path_name = dir.file_name().to_owned().into_string().unwrap();
    let depth = dir.depth();

    let first_char = if is_last { LAST_CHAR } else { LINE_CHAR };
    if depth == 0 {
        path_name
    } else {
        let indent = (depth - 1) * 4;
        format!(
            "{:indent$}{}── {}",
            "",
            first_char,
            path_name,
            indent = indent
        )
    }
}

pub fn print(dir: &DirEntry, is_last: bool, parents: &[usize]) {
    println!("{}", dir_str(dir, is_last, parents));
}
