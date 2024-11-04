use std::fmt::Write;

pub struct Formatter<'a> {
    options: crate::Options,
    defs: crate::Definitions,
    ident_depth: u8,

    buf: &'a mut (dyn Write + 'a),
}

impl<'a> Formatter<'a> {
    #[inline]
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self::new_with_options(buf, Default::default())
    }

    #[inline]
    pub fn new_with_options(buf: &'a mut (dyn Write + 'a), options: crate::Options) -> Self {
        Self {
            options,
            defs: crate::Definitions,
            ident_depth: 0,
            buf,
        }
    }

    #[inline]
    pub fn options(&self) -> &crate::Options {
        &self.options
    }

    #[inline]
    pub fn defs(&self) -> crate::Definitions {
        self.defs
    }

    #[inline]
    pub fn ident(&self) -> String {
        self.options.ident(self.ident_depth)
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
    pub fn reset_ident(&mut self) {
        self.ident_depth = 0;
    }
}

impl std::fmt::Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}
