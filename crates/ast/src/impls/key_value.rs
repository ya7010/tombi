use std::cmp::Ordering;

impl PartialOrd for crate::KeyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.keys(), other.keys()) {
            (Some(keys1), Some(keys2)) => keys1.keys().partial_cmp(keys2.keys()),
            _ => None,
        }
    }
}

impl Ord for crate::KeyValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
