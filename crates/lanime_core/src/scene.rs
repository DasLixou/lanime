use slotmap::{new_key_type, HopSlotMap};

use crate::{BoxedNode, Node, NodeRef};

new_key_type! {
    pub struct NodeIdx;
}

pub trait IntoNodeIdx {
    fn idx(&self) -> NodeIdx;
}

impl<N: Node> IntoNodeIdx for NodeRef<N> {
    #[inline]
    fn idx(&self) -> NodeIdx {
        self.idx
    }
}

pub struct SceneDescriptor<'a> {
    pub label: Option<&'a str>,
}

pub struct SceneBuilder {
    nodes: HopSlotMap<NodeIdx, BoxedNode>,
}

impl SceneBuilder {
    #[inline]
    pub fn node<N: Node + 'static>(&mut self, node: N) -> NodeRef<N> {
        let idx = self.nodes.insert(Box::new(node));
        NodeRef {
            idx,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn build(self, _descriptor: &SceneDescriptor<'_>) -> Scene {
        Scene { nodes: self.nodes }
    }
}

pub struct Scene {
    nodes: HopSlotMap<NodeIdx, BoxedNode>,
}

impl Scene {
    pub fn builder() -> SceneBuilder {
        SceneBuilder {
            nodes: HopSlotMap::with_key(),
        }
    }
}
