mod document;
mod table;
mod value;

use super::{SchemaDefinitions, ValueSchema};
use crate::Accessor;

pub trait FindCandidates {
    fn find_candidates(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<ValueSchema>, Vec<crate::Error>);
}
