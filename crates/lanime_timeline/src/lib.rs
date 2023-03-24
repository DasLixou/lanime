use std::any::Any;

use track::Track;

pub mod clip;
pub mod track;

pub struct Timeline {
    tracks: Vec<Box<dyn Any>>,
}

impl Timeline {
    pub const fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    #[inline]
    pub fn add_track<T: 'static>(&mut self, track: Track<T>) {
        self.tracks.push(Box::new(track));
    }
}
