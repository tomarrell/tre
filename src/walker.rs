use config::Options;
use ignore::{Walk, WalkBuilder};

pub fn build(opt: Options) -> Walk {
    let mut walker = WalkBuilder::new(opt.root.unwrap_or(".".to_owned()));
    walker.follow_links(opt.follow_sym_links);
    walker.max_depth(opt.max_depth);
    walker.hidden(!opt.show_hidden);
    walker.sort_by_file_name(|a, b| a.cmp(b));
    walker.build()
}
