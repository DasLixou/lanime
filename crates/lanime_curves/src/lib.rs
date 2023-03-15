use point::Point;

pub mod cubic;
pub mod linear;
pub mod point;
pub mod quadratic;

pub trait Curve {
    /// Interpolates a point on a Beziér Curve at `t` where `t` is `0..=1`
    fn interpolate(&self, t: f32) -> Point;
}

macro_rules! curve_math {
    ($t:expr, {$($start:tt)+}, {$($end:tt)+}) => {
        (1. - $t) * ($($start)+) + $t * ($($end)+)
    };
}

#[rustfmt::skip]
macro_rules! recursive_tool {
    ($me:expr, $t:expr, $start_idx:expr, 1) => {
        $me[$start_idx]
    };
    ($me:expr, $t:expr, $start_idx:expr, 2) => {
        curve_math!($t, 
            { recursive_tool!($me, $t, $start_idx, 1) },
            { recursive_tool!($me, $t, $start_idx + 1, 1)}
        )
    };
    ($me:expr, $t:expr, $start_idx:expr, 3) => {
        curve_math!($t, 
            { recursive_tool!($me, $t, $start_idx, 2) },
            { recursive_tool!($me, $t, $start_idx + 1, 2)}
        )
    };
    /*($me:expr, $t:expr, $start_idx:expr, $len:expr) => {
        curve_math!(t, 
            { recursive_tool!($me, $t, $start_idx, $len - 1) },
            { recursive_tool!($me, $t, $start_idx + 1, $len - 1)}
        )
    };*/
}

impl Curve for &[Point; 1] {
    fn interpolate(&self, _t: f32) -> Point {
        self[0]
    }
}

impl Curve for &[Point; 2] {
    fn interpolate(&self, t: f32) -> Point {
        curve_math!(t, { self[0] }, { self[1] })
    }
}

impl Curve for &[Point; 3] {
    fn interpolate(&self, t: f32) -> Point {
        recursive_tool!(self, t, 0, 3)
    }
}
