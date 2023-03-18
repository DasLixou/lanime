use crate::bindable_field::BindableField;

#[derive(Debug)]
pub struct Transform {
    pub x: f32,
    pub y: f32,
}

impl Transform {
    pub const DEFAULT: Self = Self { x: 0., y: 0. };
}

#[allow(non_upper_case_globals)]
impl Transform {
    pub const x: BindableField<Self, f32> = BindableField(|me| &mut me.x);
    pub const y: BindableField<Self, f32> = BindableField(|me| &mut me.y);
}
