use lanime_timeline::{
    track::{FindStrategy, Track},
    Timeline,
};

fn main() {
    let mut timeline = Timeline::new();
    let mut track = Track::new();
    track.add_clip((), 0, 12);
    println!("{:?}", track.find_clip_mut(11, FindStrategy::Direct));
    timeline.add_track(track);
}
