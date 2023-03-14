use crate::{point::Point, Curve};

pub struct CubicCurve {
    pub start: Point,
    pub control1: Point,
    pub control2: Point,
    pub end: Point,
}

impl Curve for CubicCurve {
    #[inline]
    fn interpolate(&self, t: f32) -> Point {
        (1. - t).powi(3) * self.start
            + 3. * (1. - t).powi(2) * t * self.control1
            + 3. * (1. - t) * t.powi(2) * self.control2
            + t.powi(3) * self.end
    }
}
