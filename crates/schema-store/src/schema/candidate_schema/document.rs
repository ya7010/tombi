use super::FindCandidates;
use crate::{Accessor, DocumentSchema, SchemaDefinitions, ValueSchema};

impl FindCandidates for DocumentSchema {
    fn find_candidates(
        &self,
        accessors: &[Accessor],
        definitions: &SchemaDefinitions,
    ) -> (Vec<ValueSchema>, Vec<crate::Error>) {
        let mut candidates = Vec::new();
        let mut errors = Vec::new();

        let Ok(mut properties) = self.properties.write() else {
            errors.push(crate::Error::DocumentLockError {
                schema_url: self.document_url.clone(),
            });
            return (candidates, errors);
        };

        if accessors.is_empty() {
            for value in properties.values_mut() {
                if let Ok(schema) = value.resolve(definitions) {
                    let (schema_candidates, schema_errors) =
                        schema.find_candidates(accessors, definitions);
                    candidates.extend(schema_candidates);
                    errors.extend(schema_errors);
                }
            }

            return (candidates, errors);
        }

        if let Some(value) = properties.get_mut(&accessors[0]) {
            if let Ok(schema) = value.resolve(&definitions) {
                return schema.find_candidates(&accessors[1..], &definitions);
            }
        }

        (candidates, errors)
    }
}
