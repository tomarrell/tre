use config::Options;
use ignore::types::TypesBuilder;
use ignore::{Error, Walk, WalkBuilder};
use std::path::Path;

/// Returns a Result for Walker based on options passed in.
pub fn build(opt: &Options) -> Result<Walk, Error> {
    let mut walker = WalkBuilder::new(opt.root.clone().unwrap_or(String::from(".")));
    let mut builder = TypesBuilder::new();

    if let Some(ref pattern) = opt.pattern {
        builder.add("custom", &format!("*{}*", pattern))?;
        builder.select("custom");
    }

    if let Some(ref extension) = opt.extension {
        builder.add("ext", &format!("*.{}", extension))?;
        builder.select("ext");
    }

    let types = builder.build()?;

    walker.types(types);
    walker.follow_links(opt.follow_sym_links);
    walker.max_depth(opt.max_depth);
    walker.hidden(!opt.show_hidden);
    walker.git_ignore(!opt.show_hidden);
    walker.sort_by_file_name(|a, b| a.cmp(b));

    Ok(walker.build())
}

/// Returns a Result for shallow Walker that has the near equilvalent search settings as normal walk.
/// It always only traverses with depth 1
pub fn build_shallow(path: &Path, opt: &Options) -> Result<Walk, Error> {
    let path = path
        .to_str()
        .expect("Invalid UTF8 path given to build_shallow")
        .to_string();

    let shallow_options = Options::new(
        Some(path),
        Some(1),
        false,
        opt.show_hidden,
        opt.dir_only,
        opt.pattern.clone(),
        opt.extension.clone(),
        opt.line_count,
        opt.no_colours,
    );

    build(&shallow_options)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Options {
        Options::new(
            Some(String::from(".")),
            Some(1),
            true,
            true,
            false,
            None,
            None,
            true,
            false,
        )
    }

    #[test]
    fn build_naive_works() {
        let walker = build(&options());

        match walker {
            Ok(_) => assert!(true),
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn build_shallow_naive_works() {
        let path = Path::new(".");
        let walker = build_shallow(path, &options());

        match walker {
            Ok(_) => assert!(true),
            Err(err) => panic!(err),
        }
    }
}
