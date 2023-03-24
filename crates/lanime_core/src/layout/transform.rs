use cgmath::{Vector2, Vector3};
use lanime_bindfields::BindFields;

#[derive(Debug, BindFields)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub size: Vector2<f32>,
}

impl Transform {
    pub const DEFAULT: Self = Self {
        position: Vector3::new(0., 0., 0.),
        size: Vector2::new(f32::MAX, f32::MAX),
    };
}
