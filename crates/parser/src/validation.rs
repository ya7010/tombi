use syntax::{SyntaxError, SyntaxNode};

pub(crate) fn validate(root: &SyntaxNode, _errors: &mut Vec<SyntaxError>) {
    let _ = _errors;
    // let _p = tracing::info_span!("parser::validate").entered();

    for _node in root.descendants() {
        continue;
    }
}
