mod boolean;

trait Format {
    fn write_fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result;
}
