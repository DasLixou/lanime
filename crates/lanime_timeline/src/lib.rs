use track::Track;

pub mod clip;
pub mod track;

pub struct Timeline {
    tracks: Vec<Track>,
}

impl Timeline {
    pub const fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    #[inline]
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }
}
