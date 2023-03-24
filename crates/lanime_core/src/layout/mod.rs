mod transform;
use cgmath::{Vector2, Vector3};
use lanime_bindfields::BindableField;
pub use transform::*;

pub struct Vector2Fields;
#[allow(non_upper_case_globals)]
impl Vector2Fields {
    pub const x: BindableField<Vector2<f32>, f32> = BindableField(|v| &mut v.x);
    pub const y: BindableField<Vector2<f32>, f32> = BindableField(|v| &mut v.y);
}

pub struct Vector3Fields;
#[allow(non_upper_case_globals)]
impl Vector3Fields {
    pub const x: BindableField<Vector3<f32>, f32> = BindableField(|v| &mut v.x);
    pub const y: BindableField<Vector3<f32>, f32> = BindableField(|v| &mut v.y);
    pub const z: BindableField<Vector3<f32>, f32> = BindableField(|v| &mut v.z);
}
