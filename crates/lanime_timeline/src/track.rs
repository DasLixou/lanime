use std::cmp::Ordering;

use crate::{clip::Clip, clip_metadata::ClipMetadata};

#[derive(Debug, PartialEq, Eq)]
pub enum FindResult<T> {
    Direct(T),
    NextLowest(T),
    Nothing,
}

pub struct Track<T> {
    clips: Vec<(ClipMetadata, Box<dyn Clip<T>>)>,
}

impl<T> Track<T> {
    pub fn new() -> Self {
        Self { clips: Vec::new() }
    }

    #[inline]
    pub fn add_clip(&mut self, clip: impl Clip<T> + 'static, start_time: usize, length: usize) {
        let meta = ClipMetadata { start_time, length };
        match self.clips.binary_search_by(|(c, _)| c.cmp(&meta)) {
            Ok(pos) => panic!("Overlapping clips at time {pos}"),
            Err(pos) => self.clips.insert(pos, (meta, Box::new(clip))),
        }
    }

    pub fn clip_index_at_time(&mut self, time: usize) -> FindResult<usize> {
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

    pub fn value_at_time(&mut self, time: usize, direct_only: bool) -> Option<T> {
        if direct_only {
            match self.clip_index_at_time(time) {
                FindResult::Direct(pos) => {
                    let clip = &mut self.clips[pos];
                    let relative_time = time - clip.0.start_time;
                    Some(clip.1.get_at(relative_time, clip.0))
                }
                FindResult::NextLowest(_) | FindResult::Nothing => None,
            }
        } else {
            match self.clip_index_at_time(time) {
                FindResult::Direct(pos) | FindResult::NextLowest(pos) => {
                    let clip = &mut self.clips[pos];
                    let relative_time = time - clip.0.start_time;
                    Some(clip.1.get_at(relative_time, clip.0))
                }
                FindResult::Nothing => None,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        clip::Clip,
        clip_metadata::ClipMetadata,
        track::{FindResult, Track},
    };

    struct ValueClip<T>(T);
    impl<T: Clone> Clip<T> for ValueClip<T> {
        #[inline] #[rustfmt::skip]
        fn get_at(&mut self, _time: usize, _meta: ClipMetadata) -> T { self.0.clone() }
    }

    #[test]
    #[should_panic]
    fn overlapping_clip() {
        let mut track = Track::new();
        track.add_clip(ValueClip('a'), 0, 20);
        track.add_clip(ValueClip('b'), 10, 30);
    }

    #[test]
    fn find_clip_at_time() {
        let mut track = Track::new();
        track.add_clip(ValueClip('a'), 5, 2); // 0
        track.add_clip(ValueClip('b'), 12, 3); // 1
        track.add_clip(ValueClip('c'), 20, 2); // 2
        assert_eq!(track.clip_index_at_time(0), FindResult::Nothing);
        assert_eq!(track.clip_index_at_time(12), FindResult::Direct(1));
        assert_eq!(track.clip_index_at_time(17), FindResult::NextLowest(1));
        assert_eq!(track.clip_index_at_time(21), FindResult::Direct(2));
        assert_eq!(track.clip_index_at_time(22), FindResult::NextLowest(2));
    }

    #[test]
    fn get_value_at_time() {
        let mut track = Track::new();
        track.add_clip(ValueClip('b'), 12, 3);
        assert_eq!(track.value_at_time(0, false), None);
        assert_eq!(track.value_at_time(12, false), Some('b'));
        assert_eq!(track.value_at_time(17, false), Some('b'));
        assert_eq!(track.value_at_time(0, true), None);
        assert_eq!(track.value_at_time(12, true), Some('b'));
        assert_eq!(track.value_at_time(17, true), None);
    }
}
