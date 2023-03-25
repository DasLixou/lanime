use lanime_timeline::{clip::Clip, clip_metadata::ClipMetadata, track::Track, Timeline};

enum AnimationClip<T: Clone> {
    InstantChange(T),
}

impl<T: Clone> Clip<T> for AnimationClip<T> {
    fn get_at(&mut self, _relative_time: usize, _meta: ClipMetadata) -> T {
        match self {
            AnimationClip::InstantChange(val) => val.clone(),
        }
    }
}

fn main() {
    let mut timeline = Timeline::new();
    let mut track: Track<f32> = Track::new();
    track.add_clip(AnimationClip::InstantChange(0.0), 0, 1);
    track.add_clip(AnimationClip::InstantChange(1.0), 10, 1);
    println!("{:?}", track.value_at_time(5, false));
    timeline.add_track(track);
}
