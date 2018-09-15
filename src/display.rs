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
    use super::*;

    #[test]
    fn it_prints_directories() {
        let dir = "test";
        let s1 = dir_str(
            dir.to_owned(),
            FileType::Directory,
            2,
            false,
            &vec![1],
            None,
        );

        assert_eq!(
            s1.to_string(),
            format!("│   ├── {}", dir.white().bold())
        );

        let s2 = dir_str(
            dir.to_owned(),
            FileType::Directory,
            5,
            true,
            &vec![1, 3],
            None,
        );

        assert_eq!(
            s2.to_string(),
            format!("│       │       └── {}", dir.white().bold())
        );

        let s3 = dir_str(dir.to_owned(), FileType::Directory, 1, true, &vec![], None);

        assert_eq!(s3.to_string(), format!("└── {}", dir.white().bold()));
    }

    #[test]
    fn it_prints_files() {
        let file = "test.rs";
        let s1 = dir_str(
            file.to_owned(),
            FileType::File,
            2,
            false,
            &vec![1],
            Some(10),
        );

        assert_eq!(
            s1.to_string(),
            format!("│   ├── {} 10", file.normal())
        );

        let s2 = dir_str(file.to_owned(), FileType::File, 5, true, &vec![1, 3], None);

        assert_eq!(
            s2.to_string(),
            format!("│       │       └── {}", file.normal())
        );

        let s3 = dir_str(file.to_owned(), FileType::File, 1, true, &vec![], None);

        assert_eq!(s3.to_string(), format!("└── {}", file.normal()));
    }

    #[test]
    fn it_prints_links() {
        let link = "test";
        let s1 = dir_str(link.to_owned(), FileType::Link, 2, false, &vec![1], None);

        assert_eq!(
            s1.to_string(),
            format!("│   ├── {}", link.green().italic())
        );

        let s2 = dir_str(link.to_owned(), FileType::Link, 5, true, &vec![1, 3], None);

        assert_eq!(
            s2.to_string(),
            format!("│       │       └── {}", link.green().italic())
        );

        let s3 = dir_str(link.to_owned(), FileType::Link, 1, true, &vec![], None);

        assert_eq!(
            s3.to_string(),
            format!("└── {}", link.green().italic())
        );
    }
}
