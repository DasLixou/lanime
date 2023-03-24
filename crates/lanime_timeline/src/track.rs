use std::cmp::Ordering;

use crate::clip::Clip;

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

    pub fn find_clip_mut(&mut self, time: usize) -> Option<&mut T> {
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
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Track;

    #[test]
    #[should_panic]
    fn overlapping_clip() {
        let mut track = Track::new();
        track.add_clip('a', 0, 20);
        track.add_clip('b', 10, 30);
    }

    #[test]
    fn find_clip_by_time() {
        let mut track = Track::new();
        track.add_clip('a', 0, 12);
        track.add_clip('b', 12, 3);
        track.add_clip('c', 20, 2);
        assert_eq!(track.find_clip_mut(0).cloned(), Some('a'));
        assert_eq!(track.find_clip_mut(12).cloned(), Some('b'));
        assert_eq!(track.find_clip_mut(21).cloned(), Some('c'));
        assert_eq!(track.find_clip_mut(22), None);
    }
}
