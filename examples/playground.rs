use lanime_timeline::{track::Track, Timeline};

#[derive(Debug)]
enum AnimationClip<T: Clone> {
    InstantChange(T),
}

fn main() {
    let mut timeline = Timeline::new();
    let mut track: Track<AnimationClip<f32>> = Track::new();
    track.add_clip(AnimationClip::InstantChange(0.0), 0, 1);
    track.add_clip(AnimationClip::InstantChange(1.0), 10, 1);
    println!("{:?}", track.clip_at_time(5));
    timeline.add_track(track);
}
