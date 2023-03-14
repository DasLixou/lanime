use crate::{point::Point, Curve};

pub struct LinearCurve {
    pub start: Point,
    pub end: Point,
}

impl Curve for LinearCurve {
    #[inline]
    fn interpolate(&self, t: f32) -> Point {
        (1. - t) * self.start + t * self.end
    }
}
