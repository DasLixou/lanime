use slotmap::{new_key_type, HopSlotMap};

use crate::{bindable_field::Lens, BoxedNode, Node, NodeOutput, NodeRef};

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

pub struct Scene {
    nodes: HopSlotMap<NodeIdx, BoxedNode>,
}

impl Scene {
    pub fn new(_descriptor: &SceneDescriptor<'_>) -> Self {
        Self {
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

    pub fn bind<'a, S, T: Node>(
        &mut self,
        source: impl NodeOutput<S>,
        desc: impl Lens<'a, Input = T, Output = S>,
        target: &NodeRef<T>,
    ) {
        // TODO:
    }
}
