use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct ClipMetadata {
    pub start_time: usize,
    pub length: usize,
}

impl ClipMetadata {
    pub(crate) fn cmp(&self, other: &Self) -> Ordering {
        if self.start_time > other.start_time + other.length {
            Ordering::Greater
        } else if self.start_time + self.length <= other.start_time {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
