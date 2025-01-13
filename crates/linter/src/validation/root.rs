use std::ops::Deref;

use schema_store::Accessor;

pub fn validate(
    root: document_tree::Root,
    document_schema: &schema_store::DocumentSchema,
) -> Result<(), Vec<crate::Error>> {
    let mut errors = Vec::new();
    let additional_properties = document_schema.additional_properties.unwrap_or_default();
    for (key, value) in root.deref().key_values() {
        if additional_properties == false
            && document_schema
                .properties
                .get(&Accessor::Key(key.to_string()))
                .is_none()
        {
            if !additional_properties {
                errors.push(crate::Error {
                    kind: crate::ErrorKind::KeyNotAllowed {
                        key: key.to_string(),
                    },
                    range: key.range() + value.range(),
                });
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
