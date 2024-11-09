use std::{borrow::Cow, fmt::Write};

pub struct Formatter<'a> {
    options: Cow<'a, crate::Options>,
    defs: crate::Definitions,
    ident_depth: u8,

    buf: &'a mut (dyn Write + 'a),
}

impl<'a> Formatter<'a> {
    #[inline]
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self {
            options: Cow::Owned(crate::Options::default()),
            defs: Default::default(),
            ident_depth: 0,
            buf,
        }
    }

    #[inline]
    pub fn new_with_options(buf: &'a mut (dyn Write + 'a), options: &'a crate::Options) -> Self {
        Self {
            options: Cow::Borrowed(options),
            defs: Default::default(),
            ident_depth: 0,
            buf,
        }
    }

    #[inline]
    pub fn options(&self) -> &crate::Options {
        &self.options
    }

    #[inline]
    pub fn defs(&self) -> &crate::Definitions {
        &self.defs
    }

    #[inline]
    pub fn line_ending(&self) -> &'static str {
        match self.options.line_ending.unwrap_or_default() {
            crate::options::LineEnding::Lf => "\n",
            crate::options::LineEnding::Crlf => "\r\n",
        }
    }

    #[inline]
    pub fn ident(&self) -> String {
        self.defs.ident(self.ident_depth)
    }

    #[inline]
    pub fn inc_ident(&mut self) {
        self.ident_depth += 1;
    }

    #[inline]
    pub fn dec_ident(&mut self) {
        self.ident_depth = self.ident_depth.saturating_sub(1);
    }

    #[inline]
    pub fn with_reset_ident(
        &mut self,
        f: impl FnOnce(&mut Self) -> Result<(), std::fmt::Error>,
    ) -> Result<(), std::fmt::Error> {
        let depth = self.ident_depth;

        self.reset_ident();

        let result = f(self);

        self.ident_depth = depth;

        result
    }

    #[inline]
    pub fn reset_ident(&mut self) {
        self.ident_depth = 0;
    }
}

impl std::fmt::Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}
