use ast::AstNode;
use config::TomlVersion;
use std::io::Read;
use toml_test::{Value, INVALID_MESSAGE};

fn main() -> Result<(), anyhow::Error> {
    let mut source = String::new();
    std::io::stdin().read_to_string(&mut source)?;

    let value = decode(&source)?;
    println!("{}", serde_json::to_string_pretty(&value).unwrap());

    Ok(())
}

fn decode(source: &str) -> Result<Value, anyhow::Error> {
    let p = parser::parse(source, TomlVersion::default());

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

    Ok(Value::from(root))
}

#[cfg(test)]
macro_rules! test_decode {
    {
        #[test]
        fn $name:ident($source:expr) -> Ok($expected:expr)
    } => {
        #[test]
        fn $name()
        {
        let source = textwrap::dedent($source);
        let value = crate::decode(source.trim()).unwrap();
        pretty_assertions::assert_eq!(
            serde_json::to_string(&value).unwrap(),
            serde_json::to_string(&$expected).unwrap()
        );
    }
    };
}

#[cfg(test)]
mod test {
    use serde_json::json;

    test_decode! {
        #[test]
        fn valid_array_array(
            r#"
            mixed = [[1, 2], ["a", "b"], [1.1, 2.1]]
            "#
        ) -> Ok(json!(
                {
                    "mixed": [
                        [
                            {"type": "integer", "value": "1"},
                            {"type": "integer", "value": "2"}
                        ],
                        [
                            {"type": "string", "value": "a"},
                            {"type": "string", "value": "b"}
                        ],
                        [
                            {"type": "float", "value": "1.1"},
                            {"type": "float", "value": "2.1"}
                        ]
                    ]
                }
            )
        )
    }
}
