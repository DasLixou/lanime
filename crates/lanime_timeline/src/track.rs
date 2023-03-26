use std::cmp::Ordering;

use crate::clip_metadata::ClipMetadata;

#[derive(Debug, PartialEq, Eq)]
pub enum FindResult<T> {
    Direct(T),
    NextLowest(T),
    Nothing,
}

pub struct Track<T> {
    clips: Vec<(ClipMetadata, T)>,
}

impl<T> Track<T> {
    pub fn new() -> Self {
        Self { clips: Vec::new() }
    }

    #[inline]
    pub fn add_clip(&mut self, clip: T, start_time: usize, length: usize) {
        let meta = ClipMetadata { start_time, length };
        match self.clips.binary_search_by(|(c, _)| c.cmp(&meta)) {
            Ok(pos) => panic!("Overlapping clips at time {pos}"),
            Err(pos) => self.clips.insert(pos, (meta, clip)),
        }
    }

    pub fn index_at_time(&self, time: usize) -> FindResult<usize> {
        let search = self.clips.binary_search_by(|(meta, _)| {
            if meta.start_time > time {
                Ordering::Greater
            } else if meta.start_time + meta.length <= time {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        match search {
            Ok(pos) => FindResult::Direct(pos),
            Err(pos) => {
                if pos > 0 {
                    FindResult::NextLowest(pos - 1)
                } else {
                    FindResult::Nothing
                }
            }
        }
    }

    pub fn clip_at_time(&self, time: usize) -> FindResult<(ClipMetadata, &T)> {
        match self.index_at_time(time) {
            FindResult::Direct(pos) => {
                let (meta, clip) = &self.clips[pos];
                FindResult::Direct((*meta, clip))
            }
            FindResult::NextLowest(pos) => {
                let (meta, clip) = &self.clips[pos];
                FindResult::NextLowest((*meta, clip))
            }
            FindResult::Nothing => FindResult::Nothing,
        }
    }

    pub fn clip_at_time_mut(&mut self, time: usize) -> FindResult<(ClipMetadata, &mut T)> {
        match self.index_at_time(time) {
            FindResult::Direct(pos) => {
                let (meta, clip) = &mut self.clips[pos];
                FindResult::Direct((*meta, clip))
            }
            FindResult::NextLowest(pos) => {
                let (meta, clip) = &mut self.clips[pos];
                FindResult::NextLowest((*meta, clip))
            }
            FindResult::Nothing => FindResult::Nothing,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::track::{FindResult, Track};

    #[test]
    #[should_panic]
    fn overlapping_clip() {
        let mut track = Track::new();
        track.add_clip('a', 0, 20);
        track.add_clip('b', 10, 30);
    }

    #[test]
    fn find_clip_at_time() {
        let mut track = Track::new();
        track.add_clip('a', 5, 2); // 0
        track.add_clip('b', 12, 3); // 1
        track.add_clip('c', 20, 2); // 2
        assert_eq!(track.index_at_time(0), FindResult::Nothing);
        assert_eq!(track.index_at_time(12), FindResult::Direct(1));
        assert_eq!(track.index_at_time(17), FindResult::NextLowest(1));
        assert_eq!(track.index_at_time(21), FindResult::Direct(2));
        assert_eq!(track.index_at_time(22), FindResult::NextLowest(2));
    }

    #[test]
    fn get_value_at_time() {
        let mut track = Track::new();
        track.add_clip('b', 12, 3);
        assert!(matches!(track.clip_at_time(0), FindResult::Nothing));
        assert!(matches!(
            track.clip_at_time(12),
            FindResult::Direct((_, &'b'))
        ));
        assert!(matches!(
            track.clip_at_time(17),
            FindResult::NextLowest((_, &'b'))
        ));
    }
}
