use std::cmp::Ordering;

use crate::clip::Clip;

#[derive(Debug, PartialEq, Eq)]
pub enum FindResult<T> {
    Direct(T),
    NextLowest(T),
    Nothing,
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

    pub fn find_clip_mut(&mut self, time: usize) -> FindResult<&mut T> {
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
            Ok(pos) => FindResult::Direct(&mut self.clips[pos].val),
            Err(pos) => {
                if pos > 0 {
                    FindResult::NextLowest(&mut self.clips[pos - 1].val)
                } else {
                    FindResult::Nothing
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::track::FindResult;

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
        track.add_clip('a', 5, 2);
        track.add_clip('b', 12, 3);
        track.add_clip('c', 20, 2);
        assert_eq!(track.find_clip_mut(0), FindResult::Nothing);
        assert_eq!(track.find_clip_mut(12), FindResult::Direct(&mut 'b'));
        assert_eq!(track.find_clip_mut(17), FindResult::NextLowest(&mut 'b'));
        assert_eq!(track.find_clip_mut(21), FindResult::Direct(&mut 'c'));
        assert_eq!(track.find_clip_mut(22), FindResult::NextLowest(&mut 'c'));
    }
}
