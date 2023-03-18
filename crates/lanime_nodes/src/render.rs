use lanime_core::{IntoNodeIdx, Node, NodeIdx};

pub struct Render {
    order: Vec<NodeIdx>,
}

impl Render {
    pub fn new(order: &[&dyn IntoNodeIdx]) -> Self {
        Render {
            order: order.iter().map(|x| x.idx()).collect(),
        }
    }
}

impl Node for Render {}
