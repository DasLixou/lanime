use crate::Res::{self, *};

pub struct Transform {
    pub x: Res<f32>,
    pub y: Res<f32>,
}

impl Transform {
    pub const DEFAULT: Self = Self {
        x: Value(0.),
        y: Value(0.),
    };
}
