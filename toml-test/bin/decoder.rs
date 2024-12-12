use ast::AstNode;
use config::TomlVersion;
use std::io::Read;
use toml_test::INVALID_MESSAGE;

fn main() -> Result<(), anyhow::Error> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;

    let p = parser::parse(&source, TomlVersion::default());

    if !p.errors().is_empty() {
        for error in p.errors() {
            eprintln!("{}", error);
        }
        return Err(anyhow::anyhow!(INVALID_MESSAGE));
    }

    let Some(root) = ast::Root::cast(p.into_syntax_node()) else {
        eprintln!("ast root cast failed");
        return Err(anyhow::anyhow!(INVALID_MESSAGE));
    };

    let root = match document_tree::Root::try_from(root) {
        Ok(root) => root,
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            return Err(anyhow::anyhow!(INVALID_MESSAGE));
        }
    };

    let document = document::Document::from(root);

    println!("{}", serde_json::to_string_pretty(&document).unwrap());

    Ok(())
}
