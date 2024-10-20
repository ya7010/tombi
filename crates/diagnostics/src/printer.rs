mod pretty;
mod simple;

pub use pretty::Pretty;
pub use simple::Simple;

pub trait Print<Printer> {
    /// Formats the object using the given formatter.
    fn print(&self, printer: Printer);
}
