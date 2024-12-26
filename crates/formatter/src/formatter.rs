pub mod definitions;

use crate::Format;
use config::{
    format::{DateTimeDelimiter, LineEnding},
    TomlVersion,
};
use diagnostic::Diagnostic;
use diagnostic::SetDiagnostics;
use std::fmt::Write;

pub struct Formatter<'a> {
    toml_version: TomlVersion,
    indent_depth: u8,
    skip_indent: bool,
    defs: crate::Definitions,
    options: &'a crate::FormatOptions,
    buf: String,
}

impl<'a> Formatter<'a> {
    #[inline]
    pub fn new(toml_version: TomlVersion, options: &'a crate::FormatOptions) -> Self {
        Self {
            toml_version,
            indent_depth: 0,
            skip_indent: false,
            defs: Default::default(),
            options: options,
            buf: String::new(),
        }
    }

    pub fn format(mut self, source: &str) -> Result<String, Vec<Diagnostic>> {
        let toml_version = self.toml_version;
        match parser::parse(source, toml_version).try_cast::<ast::Root>() {
            Ok(root) => {
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
        &self.options
    }

    #[inline]
    pub(crate) fn defs(&self) -> &crate::Definitions {
        &self.defs
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
}

impl std::fmt::Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}
