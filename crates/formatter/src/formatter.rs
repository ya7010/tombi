use std::fmt::Write;

pub struct Formatter<'a> {
    options: crate::Options,
    ident_depth: u8,

    buf: &'a mut (dyn Write + 'a),
}

impl<'a> Formatter<'a> {
    pub fn new(buf: &'a mut (dyn Write + 'a)) -> Self {
        Self::new_with_options(buf, Default::default())
    }

    pub fn new_with_options(buf: &'a mut (dyn Write + 'a), options: crate::Options) -> Self {
        Self {
            options,
            ident_depth: 0,
            buf,
        }
    }

    pub fn options(&self) -> &crate::Options {
        &self.options
    }

    pub fn ident(&self) -> String {
        self.options.ident(self.ident_depth)
    }
}

impl std::fmt::Write for Formatter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}
