use std::cmp::Ordering;

use crate::clip::Clip;

pub enum FindStrategy {
    Direct,
    NextLowest,
}

pub struct Track<T> {
    clips: Vec<Clip<T>>,
}

impl<T> Track<T> {
    pub fn new() -> Self {
        Self { clips: Vec::new() }
    }

    #[inline]
    pub fn add_clip(&mut self, val: T, start_time: usize, length: usize) {
        let clip = Clip {
            start_time,
            length,
            val,
        };
        match self.clips.binary_search_by(|c| c.cmp(&clip)) {
            Ok(pos) => panic!("Overlapping clips at time {pos}"),
            Err(pos) => self.clips.insert(pos, clip),
        }
    }

    pub fn find_clip_mut(&mut self, time: usize, strategy: FindStrategy) -> Option<&mut T> {
        let search = self.clips.binary_search_by(|clip| {
            if clip.start_time > time {
                Ordering::Greater
            } else if clip.start_time + clip.length <= time {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
        match search {
            Ok(pos) => Some(&mut self.clips[pos].val),
            Err(pos) => match strategy {
                FindStrategy::Direct => None,
                FindStrategy::NextLowest => {
                    if pos > 0 {
                        Some(&mut self.clips[pos - 1].val)
                    } else {
                        None
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::track::FindStrategy;

    use super::Track;

    #[test]
    #[should_panic]
    fn overlapping_clip() {
        let mut track = Track::new();
        track.add_clip('a', 0, 20);
        track.add_clip('b', 10, 30);
    }

    #[test]
    fn find_clip_by_time_direct() {
        let mut track = Track::new();
        track.add_clip('a', 0, 12);
        track.add_clip('b', 12, 3);
        track.add_clip('c', 20, 2);
        assert_eq!(
            track.find_clip_mut(0, FindStrategy::Direct).cloned(),
            Some('a')
        );
        assert_eq!(
            track.find_clip_mut(12, FindStrategy::Direct).cloned(),
            Some('b')
        );
        assert_eq!(
            track.find_clip_mut(21, FindStrategy::Direct).cloned(),
            Some('c')
        );
        assert_eq!(track.find_clip_mut(22, FindStrategy::Direct).cloned(), None);
    }

    #[test]
    fn find_clip_by_time_next_lowest() {
        let mut track = Track::new();
        track.add_clip('a', 5, 2);
        track.add_clip('b', 12, 3);
        track.add_clip('c', 20, 2);
        assert_eq!(
            track.find_clip_mut(0, FindStrategy::NextLowest).cloned(),
            None
        );
        assert_eq!(
            track.find_clip_mut(12, FindStrategy::NextLowest).cloned(),
            Some('b')
        );
        assert_eq!(
            track.find_clip_mut(17, FindStrategy::NextLowest).cloned(),
            Some('b')
        );
        assert_eq!(
            track.find_clip_mut(21, FindStrategy::NextLowest).cloned(),
            Some('c')
        );
        assert_eq!(
            track.find_clip_mut(22, FindStrategy::NextLowest).cloned(),
            Some('c')
        );
    }
}
