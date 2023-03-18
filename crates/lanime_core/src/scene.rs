use slotmap::{new_key_type, HopSlotMap};

use crate::{BoxedNode, Node};

pub struct SceneDescriptor<'a> {
    pub label: Option<&'a str>,
}

new_key_type! {
    pub struct NodeIdx;
}

pub struct Scene {
    nodes: HopSlotMap<NodeIdx, BoxedNode>,
}

impl Scene {
    pub fn create(_descriptor: &SceneDescriptor<'_>) -> Self {
        Scene {
            nodes: HopSlotMap::with_key(),
        }
    }

    #[inline]
    pub fn node(&mut self, node: impl Node + 'static) -> NodeIdx {
        self.nodes.insert(Box::new(node))
    }
}
