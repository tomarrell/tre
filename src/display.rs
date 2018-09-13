use ignore::DirEntry;

const PARENT_CHAR: &str = "│";
const LAST_CHAR: &str = "└";
const LINE_CHAR: &str = "├";

pub fn dir_str(
    file_name: String,
    depth: usize,
    is_last: bool,
    parents: &Vec<usize>,
    line_count: Option<usize>,
) -> String {
    let first_char = if is_last { LAST_CHAR } else { LINE_CHAR };
    let print_path = match line_count {
        Some(count) => format!("{} {}", file_name, count),
        _ => file_name,
    };

    if depth == 0 {
        print_path
    } else {
        let mut prefix = String::new();

        for i in 1..depth {
            prefix = format!(
                "{}{:space$}",
                prefix,
                if parents.contains(&i) || i == 1 {
                    PARENT_CHAR
                } else {
                    " "
                },
                space = 4
            );
        }

        format!("{}{}── {}", prefix, first_char, print_path)
    }
}

pub fn print(dir: &DirEntry, is_last: bool, parents: &Vec<usize>, line_count: Option<usize>) {
    let file_name = dir.file_name().to_owned().into_string().unwrap();
    let depth = dir.depth();
    println!(
        "{}",
        dir_str(file_name, depth, is_last, parents, line_count)
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("it works", "it works");
    }
}
