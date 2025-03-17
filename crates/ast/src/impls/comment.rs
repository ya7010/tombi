use crate::Comment;

impl Comment {
    pub fn schema_url(&self, source_path: Option<&std::path::Path>) -> Option<url::Url> {
        let comment_string = self.to_string();
        if comment_string.starts_with("#:schema ") {
            let url_str = comment_string[9..].trim();
            if let Ok(url) = url_str.parse::<url::Url>() {
                Some(url)
            } else if let Some(source_dir_path) = source_path {
                url::Url::from_file_path(source_dir_path)
                    .ok()
                    .and_then(|url| url.join(url_str).ok())
            } else {
                None
            }
        } else {
            None
        }
    }
}
