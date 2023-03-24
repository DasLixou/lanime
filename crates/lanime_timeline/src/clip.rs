use std::cmp::Ordering;

#[derive(Debug)]
pub(crate) struct Clip<T> {
    pub(crate) start_time: usize,
    pub(crate) length: usize,
    pub(crate) val: T,
}

impl<T> Clip<T> {
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
