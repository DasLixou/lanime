use std::cmp::Ordering;

use crate::clip::Clip;

pub struct Track {
    clips: Vec<Clip>,
}

impl Track {
    pub fn new() -> Self {
        Self { clips: Vec::new() }
    }

    #[inline]
    pub fn add_clip(&mut self, start_time: usize, length: usize) {
        let clip = Clip { start_time, length };
        match self.clips.binary_search(&clip) {
            Ok(pos) => panic!("Overlapping clips at time {pos}"),
            Err(pos) => self.clips.insert(pos, clip),
        }
    }

    pub fn find_clip(&self, time: usize) -> Result<usize, usize> {
        self.clips.binary_search_by(|clip| {
            if clip.start_time > time {
                Ordering::Greater
            } else if clip.start_time + clip.length <= time {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::Track;

    #[test]
    #[should_panic]
    fn overlapping_clip() {
        let mut track = Track::new();
        track.add_clip(0, 20);
        track.add_clip(10, 30);
    }

    #[test]
    fn find_clip_by_time() {
        let mut track = Track::new();
        track.add_clip(0, 12); // 0
        track.add_clip(12, 3); // 1
        track.add_clip(20, 2); // 2
        assert_eq!(track.find_clip(0), Ok(0));
        assert_eq!(track.find_clip(12), Ok(1));
        assert_eq!(track.find_clip(21), Ok(2));
        assert!(track.find_clip(22).is_err());
    }
}
