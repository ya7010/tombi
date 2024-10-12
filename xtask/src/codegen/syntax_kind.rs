use quote::quote;
use ungrammar::Grammar;

pub fn generate_syntax_kind(_grammer: &Grammar) -> Result<String, anyhow::Error> {
    let token = quote! {
        #![allow(clippy::all)]
    };

    crate::utils::reformat(token)
}
