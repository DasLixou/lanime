use std::any::Any;

use slotmap::new_key_type;
use sorted_list::SortedList;
use track::Track;

pub mod clip;
pub mod clip_metadata;
pub mod sorted_list;
pub mod track;

new_key_type! { pub struct TrackId; }

pub struct Timeline {
    tracks: SortedList<TrackId, Box<dyn Any>>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            tracks: SortedList::new(),
        }
    }

    #[inline]
    pub fn add_track<T: 'static>(&mut self, track: Track<T>) -> TrackId {
        self.tracks.insert_top(Box::new(track))
    }
}
