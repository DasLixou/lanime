use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Clip {
    pub(crate) start_time: usize,
    pub(crate) length: usize,
}

impl PartialOrd for Clip {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Clip {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.start_time > other.start_time + other.length {
            Ordering::Greater
        } else if self.start_time + self.length <= other.start_time {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
