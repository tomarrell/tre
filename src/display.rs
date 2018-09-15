use colored::*;
use stats_collector::FileType;

const PARENT_CHAR: &str = "│";
const LAST_CHAR: &str = "└";
const LINE_CHAR: &str = "├";

pub fn dir_str(
    file_name: String,
    file_type: FileType,
    depth: usize,
    is_last: bool,
    parents: &Vec<usize>,
    line_count: Option<usize>,
) -> ColoredString {
    let first_char = if is_last { LAST_CHAR } else { LINE_CHAR };

    let print_path = match line_count {
        Some(count) => format!("{} {}", file_name, count),
        _ => file_name,
    };

    let print_path = match file_type {
        FileType::Directory => print_path.white().bold(),
        FileType::Link => print_path.green().italic(),
        _ => print_path.white().normal(),
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

        format!("{}{}── {}", prefix, first_char, print_path).normal()
    }
}

pub fn print(
    file_name: String,
    file_type: FileType,
    depth: usize,
    is_last: bool,
    parents: &Vec<usize>,
    line_count: Option<usize>,
) {
    println!(
        "{}",
        dir_str(file_name, file_type, depth, is_last, parents, line_count)
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("it works", "it works");
    }
}
