mod tag;

pub use tag::Tag;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Highlight {
    pub tag: Tag,
}
