use crate::{AstToken, Comment};

impl Comment {
    pub fn schema_url(
        &self,
        source_path: Option<&std::path::Path>,
    ) -> Option<(url::Url, text::Range)> {
        let comment_string = self.to_string();
        if comment_string.starts_with("#:schema ") {
            let url_str = comment_string[9..].trim();
            let mut comment_range = self.syntax().range();
            comment_range = text::Range::new(
                text::Position::new(comment_range.start().line(), 9),
                text::Position::new(
                    comment_range.end().line(),
                    9 + url_str.len() as text::Column,
                ),
            );
            if let Ok(url) = url_str.parse::<url::Url>() {
                Some((url, comment_range))
            } else if let Some(source_dir_path) = source_path {
                url::Url::from_file_path(source_dir_path)
                    .ok()
                    .and_then(|url| {
                        url.join(url_str)
                            .map(|joined_url| (joined_url, comment_range))
                            .ok()
                    })
            } else {
                None
            }
        } else {
            None
        }
    }
}
