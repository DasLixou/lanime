use slotmap::{new_key_type, HopSlotMap};

use crate::{BoxedNode, Node, NodeRef};

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
    pub fn node<N: Node + 'static>(&mut self, node: N) -> NodeRef<N> {
        let idx = self.nodes.insert(Box::new(node));
        NodeRef {
            idx,
            phantom: std::marker::PhantomData,
        }
    }
}
