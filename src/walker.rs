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
    let mut walker = WalkBuilder::new(path);
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
    walker.max_depth(Some(1));
    walker.hidden(!opt.show_hidden);
    walker.git_ignore(!opt.show_hidden);
    walker.sort_by_file_name(|a, b| a.cmp(b));

    Ok(walker.build())
}

#[cfg(test)]
mod tests {}
