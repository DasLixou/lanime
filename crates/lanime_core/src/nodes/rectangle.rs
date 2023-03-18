use crate::{Node, Res};

pub struct Rectangle {
    pub width: Res<f32>,
    pub height: Res<f32>,
}

impl Node for Rectangle {}
