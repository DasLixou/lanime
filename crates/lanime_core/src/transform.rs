use lanime_bindfields::BindFields;

#[derive(Debug, BindFields)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
}

impl Transform {
    pub const DEFAULT: Self = Self { x: 0., y: 0. };
}
