#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Offset {
    /// > A suffix which, when applied to a time, denotes a UTC offset of 00:00;
    /// > often spoken "Zulu" from the ICAO phonetic alphabet representation of
    /// > the letter "Z". --- [RFC 3339 section 2]
    ///
    /// [RFC 3339 section 2]: https://datatracker.ietf.org/doc/html/rfc3339#section-2
    Z,

    /// Offset between local time and UTC
    Custom {
        /// Minutes: -`1_440..1_440`
        minutes: i16,
    },
}

impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Offset::Z => write!(f, "Z"),
            Offset::Custom { mut minutes } => {
                let mut sign = '+';
                if minutes < 0 {
                    minutes *= -1;
                    sign = '-';
                }
                let hours = minutes / 60;
                let minutes = minutes % 60;
                write!(f, "{sign}{hours:02}:{minutes:02}")
            }
        }
    }
}
