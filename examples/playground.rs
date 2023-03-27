use lanime_timeline::{
    animation_curve::AnimationCurve,
    track::{FindResult, Track},
    Timeline,
};

enum AnimationClip {
    InstantChange(f32),
    Curve(AnimationCurve),
}

impl AnimationClip {
    pub fn interpolate(&self, percent: f32) -> f32 {
        match self {
            AnimationClip::InstantChange(val) => *val,
            AnimationClip::Curve(curve) => curve.curve_y(percent),
        }
    }

    #[inline]
    pub fn end_val(&self) -> f32 {
        self.interpolate(1.)
    }
}

fn main() {
    let mut timeline = Timeline::new();
    let mut track: Track<AnimationClip> = Track::new();
    track.add_clip(AnimationClip::InstantChange(0.0), 0, 1);
    track.add_clip(AnimationClip::Curve(AnimationCurve::ease_out()), 5, 5);
    println!("{:?}", clip_helper(&track, 8));
    timeline.add_track(track);
}

fn clip_helper(track: &Track<AnimationClip>, time: usize) -> f32 {
    match track.clip_at_time(time) {
        FindResult::Direct((meta, clip)) => {
            let percent = (time - meta.start_time) as f32 / meta.length as f32;
            clip.interpolate(percent)
        }
        FindResult::NextLowest((_meta, clip)) => clip.end_val(),
        FindResult::Nothing => panic!("Initial Value isn't known for this curve at frame '{time}'"),
    }
}
