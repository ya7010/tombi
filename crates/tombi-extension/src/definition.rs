use tower_lsp::lsp_types::request::GotoDeclarationResponse;

#[derive(Debug)]
pub struct DefinitionLocation {
    pub uri: tower_lsp::lsp_types::Url,
    pub range: tombi_text::Range,
}

impl DefinitionLocation {
    pub fn new(uri: tower_lsp::lsp_types::Url, range: tombi_text::Range) -> Self {
        Self { uri, range }
    }
}

impl From<DefinitionLocation> for tower_lsp::lsp_types::Location {
    fn from(definition_location: DefinitionLocation) -> Self {
        tower_lsp::lsp_types::Location::new(
            definition_location.uri,
            definition_location.range.into(),
        )
    }
}

impl Into<GotoDeclarationResponse> for DefinitionLocation {
    fn into(self) -> GotoDeclarationResponse {
        GotoDeclarationResponse::Scalar(self.into())
    }
}

impl Into<DefinitionLocations> for DefinitionLocation {
    fn into(self) -> DefinitionLocations {
        DefinitionLocations(vec![self])
    }
}

#[derive(Debug)]
pub struct DefinitionLocations(pub Vec<DefinitionLocation>);

impl Into<Option<GotoDeclarationResponse>> for DefinitionLocations {
    fn into(self) -> Option<GotoDeclarationResponse> {
        match self.0.len() {
            0 => None,
            1 => Some(GotoDeclarationResponse::Scalar(
                self.0.into_iter().next().unwrap().into(),
            )),
            _ => Some(GotoDeclarationResponse::Array(
                self.0.into_iter().map(|location| location.into()).collect(),
            )),
        }
    }
}
