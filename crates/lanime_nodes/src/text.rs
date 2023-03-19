use lanime_bindfields::BindFields;
use lanime_core::{Node, Transform};

#[derive(Debug, BindFields)]
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

impl Node for Text<'static> {}
