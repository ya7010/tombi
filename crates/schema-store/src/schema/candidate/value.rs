use crate::{Accessor, SchemaDefinitions, ValueSchema};

use super::FindCandidates;

impl FindCandidates for ValueSchema {
    fn find_candidates(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<ValueSchema>, Vec<crate::Error>) {
        if accessors.is_empty() {
            (vec![self.clone()], Vec::new())
        } else {
            match self {
                Self::Table(table) => table.find_candidates(accessors, definitions),
                _ => (Vec::new(), Vec::new()),
            }
        }
    }
}
