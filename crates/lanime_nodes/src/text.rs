use lanime_core::{bindable_field::BindableField, Node, Transform};

#[derive(Debug)]
pub struct Text<'s> {
    pub text: &'s str,
    pub transform: Transform,
}

impl Text<'_> {
    pub const DEFAULT: Self = Self {
        text: "",
        transform: Transform::DEFAULT,
    };
}

#[allow(non_upper_case_globals)]
impl<'s> Text<'s> {
    pub const text: BindableField<Self, &'s str> = BindableField(|me| &mut me.text);
    pub const transform: BindableField<Self, Transform> = BindableField(|me| &mut me.transform);
}

impl Node for Text<'static> {}
