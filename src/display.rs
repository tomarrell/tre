use ignore::DirEntry;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

const PARENT_CHAR: &str = "│";
const LAST_CHAR: &str = "└";
const LINE_CHAR: &str = "├";

pub fn dir_str(
    path: String,
    depth: usize,
    is_last: bool,
    parents: &Vec<usize>,
    line_count: Option<usize>,
) -> String {
    let first_char = if is_last { LAST_CHAR } else { LINE_CHAR };
    let print_path = match line_count {
        Some(count) => format!("{} {}", path, count),
        _ => path,
    };

    if depth == 0 {
        print_path
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

        format!("{}{}── {}", indent, first_char, print_path)
    }
}

pub fn print(
    dir: &DirEntry,
    is_last: bool,
    parents: &Vec<usize>,
    count_lines: bool,
) -> Result<(usize), Error> {
    let file_name = dir.file_name().to_owned().into_string().unwrap();
    let depth = dir.depth();
    let lines = match (dir.file_type(), count_lines) {
        (Some(typ), true) if typ.is_file() => {
            let mut f = File::open(dir.path())?;
            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => (),
                Err(_) => (),
            };

            Some(s.lines().count())
        }
        _ => None,
    };
    println!("{}", dir_str(file_name, depth, is_last, parents, lines));
    Ok(lines.unwrap_or(0))
}

pub fn print_stats(
    files: usize,
    directories: usize,
    links: usize,
    lines: usize,
    print_count: bool,
) {
    println!(
        "\n{} directories, {} files, {} symbolic links{}",
        directories,
        files,
        links,
        if print_count {
            format!(", {} lines", lines)
        } else {
            String::from("")
        }
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!("it works", "it works");
    }
}

