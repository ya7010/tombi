use std::{str::FromStr, sync::Arc};

use itertools::Itertools;

use super::SchemaUri;
use crate::JsonSchemaDialect;

#[derive(Debug, Clone)]
pub(crate) struct SchemaResource {
    /// **schema_resource_uri**: Effective identity of a schema resource (`$id` when present,
    /// otherwise `schema_document_uri`).
    pub schema_resource_uri: SchemaUri,
    pub id: Option<SchemaUri>,
    pub dialect: Option<JsonSchemaDialect>,
    /// JSON Pointer to this resource within the physical document (e.g. `#/$defs/foo`).
    pub location: String,
    pub value: tombi_json::ValueNode,
}

#[derive(Debug)]
pub(crate) struct SchemaDocumentResources {
    /// **schema_document_uri**: Physical URI of the loaded JSON Schema document (file/http/etc).
    /// Not `$id`.
    schema_document_uri: SchemaUri,
    root_schema_resource_uri: SchemaUri,
    resources: tombi_hashmap::HashMap<SchemaUri, SchemaResource>,
}

impl SchemaDocumentResources {
    pub(crate) fn collect(
        root: &tombi_json::ValueNode,
        schema_document_uri: &SchemaUri,
    ) -> Result<Arc<Self>, crate::Error> {
        let mut resources = tombi_hashmap::HashMap::default();
        let root_schema_resource_uri = collect_schema_resources_from_value(
            root,
            schema_document_uri,
            schema_document_uri,
            None,
            true,
            "#",
            &mut resources,
        )?
        .expect("the root schema always defines a schema resource");

        if schema_document_uri != &root_schema_resource_uri
            && let Some(existing) = resources.get(schema_document_uri)
        {
            return Err(crate::Error::DuplicateSchemaResourceInDocument {
                schema_uri: schema_document_uri.clone(),
                schema_document_uri: schema_document_uri.clone(),
                first_location: existing.location.clone(),
                second_location: "#".to_string(),
            });
        }

        Ok(Arc::new(Self {
            schema_document_uri: schema_document_uri.clone(),
            root_schema_resource_uri,
            resources,
        }))
    }

    pub(crate) fn schema_document_uri(&self) -> &SchemaUri {
        &self.schema_document_uri
    }

    pub(crate) fn root_schema_resource_uri(&self) -> &SchemaUri {
        &self.root_schema_resource_uri
    }

    pub(crate) fn resource(&self, schema_resource_uri: &SchemaUri) -> Option<&SchemaResource> {
        self.resources.get(schema_resource_uri)
    }

    pub(crate) fn aliases(&self) -> Vec<(SchemaUri, SchemaUri)> {
        let mut aliases = self
            .resources
            .keys()
            .map(|schema_resource_uri| (schema_resource_uri.clone(), schema_resource_uri.clone()))
            .collect_vec();
        if self.schema_document_uri != self.root_schema_resource_uri {
            aliases.push((
                self.schema_document_uri.clone(),
                self.root_schema_resource_uri.clone(),
            ));
        }
        aliases
    }
}

fn collect_schema_resources_from_value(
    value: &tombi_json::ValueNode,
    schema_document_uri: &SchemaUri,
    enclosing_schema_base_uri: &SchemaUri,
    inherited_dialect: Option<JsonSchemaDialect>,
    is_document_root: bool,
    location: &str,
    resources: &mut tombi_hashmap::HashMap<SchemaUri, SchemaResource>,
) -> Result<Option<SchemaUri>, crate::Error> {
    let tombi_json::ValueNode::Object(object) = value else {
        if is_document_root {
            let schema_resource_uri = schema_document_uri.clone();
            resources.insert(
                schema_resource_uri.clone(),
                SchemaResource {
                    schema_resource_uri: schema_resource_uri.clone(),
                    id: None,
                    dialect: inherited_dialect,
                    location: location.to_string(),
                    value: value.clone(),
                },
            );
            return Ok(Some(schema_resource_uri));
        }
        return Ok(None);
    };

    let declared_schema_resource_uri = object
        .get("$id")
        .and_then(tombi_json::ValueNode::as_str)
        .and_then(|id| resolve_schema_resource_uri(enclosing_schema_base_uri, id));
    let dialect = (is_document_root || declared_schema_resource_uri.is_some())
        .then(|| {
            object
                .get("$schema")
                .and_then(tombi_json::ValueNode::as_str)
                .and_then(|uri| JsonSchemaDialect::try_from(uri).ok())
        })
        .flatten()
        .or(inherited_dialect);
    let schema_resource_uri = declared_schema_resource_uri
        .clone()
        .or_else(|| is_document_root.then(|| schema_document_uri.clone()));
    let schema_base_uri = declared_schema_resource_uri
        .as_ref()
        .unwrap_or(enclosing_schema_base_uri);

    if let Some(schema_resource_uri) = &schema_resource_uri {
        if let Some(existing) = resources.get(schema_resource_uri) {
            return Err(crate::Error::DuplicateSchemaResourceInDocument {
                schema_uri: schema_resource_uri.clone(),
                schema_document_uri: schema_document_uri.clone(),
                first_location: existing.location.clone(),
                second_location: location.to_string(),
            });
        }
        resources.insert(
            schema_resource_uri.clone(),
            SchemaResource {
                schema_resource_uri: schema_resource_uri.clone(),
                id: declared_schema_resource_uri.clone(),
                dialect,
                location: location.to_string(),
                value: value.clone(),
            },
        );
    }

    for (key, child) in &object.properties {
        match key.value.as_str() {
            "$defs" | "definitions" | "properties" | "patternProperties" | "dependentSchemas" => {
                if let Some(children) = child.as_object() {
                    for (child_key, child) in &children.properties {
                        collect_schema_resources_from_value(
                            child,
                            schema_document_uri,
                            schema_base_uri,
                            dialect,
                            false,
                            &json_pointer_join(
                                location,
                                key.value.as_str(),
                                Some(&child_key.value),
                            ),
                            resources,
                        )?;
                    }
                }
            }
            "dependencies" => {
                if let Some(children) = child.as_object() {
                    for (child_key, child) in children.properties.iter().filter(|(_, value)| {
                        matches!(
                            value,
                            tombi_json::ValueNode::Object(_) | tombi_json::ValueNode::Bool(_)
                        )
                    }) {
                        collect_schema_resources_from_value(
                            child,
                            schema_document_uri,
                            schema_base_uri,
                            dialect,
                            false,
                            &json_pointer_join(
                                location,
                                key.value.as_str(),
                                Some(&child_key.value),
                            ),
                            resources,
                        )?;
                    }
                }
            }
            "allOf" | "anyOf" | "oneOf" | "prefixItems" => {
                if let Some(children) = child.as_array() {
                    for (index, child) in children.items.iter().enumerate() {
                        collect_schema_resources_from_value(
                            child,
                            schema_document_uri,
                            schema_base_uri,
                            dialect,
                            false,
                            &json_pointer_join(
                                location,
                                key.value.as_str(),
                                Some(&index.to_string()),
                            ),
                            resources,
                        )?;
                    }
                }
            }
            "items" => {
                if let Some(children) = child.as_array() {
                    for (index, child) in children.items.iter().enumerate() {
                        collect_schema_resources_from_value(
                            child,
                            schema_document_uri,
                            schema_base_uri,
                            dialect,
                            false,
                            &json_pointer_join(
                                location,
                                key.value.as_str(),
                                Some(&index.to_string()),
                            ),
                            resources,
                        )?;
                    }
                } else {
                    collect_schema_resources_from_value(
                        child,
                        schema_document_uri,
                        schema_base_uri,
                        dialect,
                        false,
                        &json_pointer_join(location, key.value.as_str(), None),
                        resources,
                    )?;
                }
            }
            "additionalItems"
            | "additionalProperties"
            | "unevaluatedItems"
            | "unevaluatedProperties"
            | "contains"
            | "propertyNames"
            | "not"
            | "if"
            | "then"
            | "else"
            | "contentSchema" => {
                collect_schema_resources_from_value(
                    child,
                    schema_document_uri,
                    schema_base_uri,
                    dialect,
                    false,
                    &json_pointer_join(location, key.value.as_str(), None),
                    resources,
                )?;
            }
            _ => {}
        }
    }

    Ok(schema_resource_uri)
}

fn escape_json_pointer_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn json_pointer_join(parent: &str, key: &str, child: Option<&str>) -> String {
    let mut pointer = if parent == "#" {
        format!("#/{}", escape_json_pointer_token(key))
    } else {
        format!("{parent}/{}", escape_json_pointer_token(key))
    };
    if let Some(child) = child {
        pointer.push('/');
        pointer.push_str(&escape_json_pointer_token(child));
    }
    pointer
}

pub(crate) fn resolve_schema_resource_uri(
    schema_base_uri: &SchemaUri,
    id: &str,
) -> Option<SchemaUri> {
    let mut schema_resource_uri = if let Ok(uri) = schema_base_uri.join(id) {
        SchemaUri::from(uri)
    } else {
        SchemaUri::from_str(id).ok()?
    };
    if schema_resource_uri
        .fragment()
        .is_some_and(|fragment| !fragment.is_empty())
    {
        return None;
    }
    schema_resource_uri.set_fragment(None);
    Some(schema_resource_uri)
}
