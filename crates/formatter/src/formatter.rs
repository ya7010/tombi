pub mod definitions;

use crate::Format;
use config::{DateTimeDelimiter, LineEnding, TomlVersion};
use diagnostic::Diagnostic;
use diagnostic::SetDiagnostics;
use itertools::Either;
use schema_store::DocumentSchema;
use std::fmt::Write;
use url::Url;

pub struct Formatter<'a> {
    toml_version: TomlVersion,
    indent_depth: u8,
    skip_indent: bool,
    defs: crate::Definitions,
    options: &'a crate::FormatOptions,
    #[allow(dead_code)]
    schema: Option<DocumentSchema>,
    #[allow(dead_code)]
    schema_store: &'a schema_store::SchemaStore,
    buf: String,
}

impl<'a> Formatter<'a> {
    #[inline]
    pub async fn try_new(
        toml_version: TomlVersion,
        options: &'a crate::FormatOptions,
        source_url_or_path: Option<Either<&'a Url, &'a std::path::Path>>,
        schema_store: &'a schema_store::SchemaStore,
    ) -> Result<Self, schema_store::Error> {
        let schema = match source_url_or_path {
            Some(source_url_or_path) => {
                Some(schema_store.try_get_schema(source_url_or_path).await?)
            }
            None => None,
        }
        .flatten();

        let toml_version = schema
            .as_ref()
            .map(|s| s.toml_version())
            .flatten()
            .unwrap_or(toml_version);

        Ok(Self {
            toml_version,
            indent_depth: 0,
            skip_indent: false,
            defs: Default::default(),
            options,
            schema,
            schema_store,
            buf: String::new(),
        })
    }

    /// Format a node and return the result as a string
    pub(crate) fn format_to_string<T: Format>(
        &mut self,
        node: &T,
    ) -> Result<String, std::fmt::Error> {
        let old_buf = std::mem::take(&mut self.buf);
        let old_indent = self.indent_depth;
        let old_skip = self.skip_indent;

        node.fmt(self)?;
        let result = std::mem::take(&mut self.buf);

        self.buf = old_buf;
        self.indent_depth = old_indent;
        self.skip_indent = old_skip;

        Ok(result)
    }

    pub async fn format(mut self, source: &str) -> Result<String, Vec<Diagnostic>> {
        match parser::parse(source, self.toml_version).try_cast::<ast::Root>() {
            Ok(root) => {
                let root = ast_editor::edit(root);

                tracing::trace!("TOML AST: {:#?}", root);

                let line_ending = {
                    root.fmt(&mut self).unwrap();
                    self.line_ending()
                };

                Ok(self.buf + line_ending)
            }
            Err(errors) => {
                let mut diagnostics = Vec::new();
                for error in errors {
                    error.set_diagnostic(&mut diagnostics);
                }
                Err(diagnostics)
            }
        }
    }

    #[inline]
    pub(crate) fn toml_version(&self) -> TomlVersion {
        self.toml_version
    }

    #[inline]
    pub(crate) fn options(&self) -> &crate::FormatOptions {
        self.options
    }

    #[inline]
    pub(crate) fn defs(&self) -> &crate::Definitions {
        &self.defs
    }

    #[inline]
    pub(crate) fn line_width(&self) -> u8 {
        self.options.line_width.unwrap_or_default().value()
    }

    #[inline]
    pub(crate) fn line_ending(&self) -> &'static str {
        match self.options.line_ending.unwrap_or_default() {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
        }
    }

    #[inline]
    pub(crate) fn date_time_delimiter(&self) -> Option<&'static str> {
        match self.options.date_time_delimiter() {
            DateTimeDelimiter::T => Some("T"),
            DateTimeDelimiter::Space => Some(" "),
            DateTimeDelimiter::Preserve => None,
        }
    }

    #[inline]
    pub(crate) fn reset(&mut self) {
        self.reset_indent();
    }

    #[inline]
    pub(crate) fn write_indent(&mut self) -> Result<(), std::fmt::Error> {
        if self.skip_indent {
            self.skip_indent = false;

            Ok(())
        } else {
            write!(self, "{}", self.options().ident(self.indent_depth))
        }
    }

    #[inline]
    pub(crate) fn inc_indent(&mut self) {
        self.indent_depth += 1;
    }

    #[inline]
    pub(crate) fn dec_indent(&mut self) {
        self.indent_depth = self.indent_depth.saturating_sub(1);
    }

    #[inline]
    fn reset_indent(&mut self) {
        self.indent_depth = 0;
    }

    #[inline]
    pub(crate) fn skip_indent(&mut self) {
        self.skip_indent = true;
    }

    #[inline]
    pub(crate) fn current_line_width(&self) -> usize {
        self.buf.split("\n").last().unwrap_or_default().len()
    }
}

impl std::fmt::Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}
