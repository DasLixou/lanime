use lanime_curves_macros::recursive_tool;
use point::Point;

pub mod point;

pub trait Curve {
    /// Interpolates a point on a Beziér Curve at `t` where `t` is `0..=1`
    fn interpolate(&self, t: f32) -> Point;
}

macro_rules! impl_curve {
    ($size:literal) => {
        impl Curve for &[Point; $size] {
            #[allow(unused)] // for `$size == 1`, `t` will be unused
            fn interpolate(&self, t: f32) -> Point {
                recursive_tool!(0, $size)
            }
        }
    };
}

impl_curve!(1);
impl_curve!(2);
impl_curve!(3);
impl_curve!(4);
impl_curve!(5);
impl_curve!(6);
impl_curve!(7);
impl_curve!(8);
