use std::path::PathBuf;

/// Input source for TOML files.
///
/// Standard input or file paths. Contains a list of files that match the glob pattern.
#[derive(Debug)]
pub enum FileInput {
    Stdin,
    Files(Vec<Result<PathBuf, Error>>),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Invalid glob pattern: {0}")]
    GlobPatternInvalid(String),
}

impl<T> From<&[T]> for FileInput
where
    T: AsRef<str>,
{
    fn from(files: &[T]) -> Self {
        match files.len() {
            0 => {
                let grob_pattern = "*/*.toml";

                FileInput::Files(
                    glob::glob(grob_pattern)
                        .unwrap()
                        .filter_map(|x| Result::<_, Error>::Ok(x.ok()).transpose())
                        .collect::<Vec<_>>(),
                )
            }
            1 if files[0].as_ref() == "-" => FileInput::Stdin,
            _ => {
                let mut results: Vec<Result<PathBuf, Error>> = Vec::with_capacity(files.len());
                for file in files {
                    if is_glob_pattern(file.as_ref()) {
                        if let Ok(paths) = glob::glob(file.as_ref()) {
                            results.extend(
                                paths
                                    .filter_map(|x| Result::<_, Error>::Ok(x.ok()).transpose())
                                    .collect::<Vec<_>>(),
                            );
                        } else {
                            results.push(Err(Error::GlobPatternInvalid(file.as_ref().into())));
                        };
                    } else {
                        let path = PathBuf::from(file.as_ref());
                        if !path.exists() {
                            results.push(Err(Error::FileNotFound(file.as_ref().into())));
                        }
                        results.push(Ok(path));
                    }
                }
                FileInput::Files(results)
            }
        }
    }
}

fn is_glob_pattern(value: &str) -> bool {
    value.contains('*') || value.contains('?') || value.contains('[') || value.contains(']')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn std_input() {
        let input = vec!["-"];
        let file_input = FileInput::from(input.as_ref());
        assert!(matches!(file_input, FileInput::Stdin));
    }

    #[test]
    fn single_file() {
        let input = vec!["Cargo.toml"];
        let file_input = FileInput::from(input.as_ref());
        assert!(matches!(file_input, FileInput::Files(_)));
    }
}
