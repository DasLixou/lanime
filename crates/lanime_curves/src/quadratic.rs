use crate::{point::Point, Curve};

pub struct QuadraticCurve {
    pub start: Point,
    pub control: Point,
    pub end: Point,
}

impl Curve for QuadraticCurve {
    #[inline]
    fn interpolate(&self, t: f32) -> Point {
        self.control
            + (1. - t).powi(2) * (self.start - self.control)
            + t.powi(2) * (self.end - self.control)
    }
}
