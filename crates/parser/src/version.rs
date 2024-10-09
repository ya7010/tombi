pub enum Version {
    V1_0,
}

pub struct Parser<'p> {
    version: Version,
    builder: rowan::GreenNodeBuilder<'p>,
}
