use lanime_core::{
    Node,
    Res::{self, *},
    Transform,
};

pub struct Text<'s> {
    pub text: Res<&'s str>,
    pub transform: Res<Transform>,
}

impl Text<'_> {
    pub const DEFAULT: Self = Self {
        text: Value(""),
        transform: Value(Transform::DEFAULT),
    };
}

impl Node for Text<'_> {}
