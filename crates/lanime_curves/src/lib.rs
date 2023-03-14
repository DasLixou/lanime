use point::Point;

pub mod cubic;
pub mod linear;
pub mod point;
pub mod quadratic;

pub trait Curve {
    /// Interpolates a point on a Beziér Curve at `t` where `t` is `0..=1`
    fn interpolate(&self, t: f32) -> Point;
}
