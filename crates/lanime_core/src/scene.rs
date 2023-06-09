use std::{any::Any, fmt::Debug, ptr};

use lanime_bindfields::Lens;
use lanime_graph::{
    post_order_dfs::{PostOrderDFS, StartNode},
    Graph, NodeIdx,
};

use crate::{BoxedNode, Node, NodeRef, NodeResult, SceneContext};

pub trait IntoNodeIdx {
    fn idx(&self) -> NodeIdx;
}

pub struct SceneDescriptor<'a> {
    pub label: Option<&'a str>,
}

type UpdateSignature = fn(&mut BoxedNode, &mut BoxedNode, &mut Box<dyn Any>, &SceneContext);

pub struct Scene {
    nodes: Graph<BoxedNode, (Box<dyn Any>, UpdateSignature)>,
}

impl Scene {
    pub fn new(_descriptor: &SceneDescriptor<'_>) -> Self {
        Self {
            nodes: Graph::new(),
        }
    }

    #[inline]
    pub fn node<N: Node + 'static>(&mut self, node: N) -> NodeRef<N> {
        let idx = self.nodes.add_node(Box::new(node));
        NodeRef {
            idx,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn bind<
        Type: 'static,
        Source: Node + NodeResult<Type> + 'static,
        Target: Node + 'static,
        L: 'static,
    >(
        &mut self,
        source: &NodeRef<Source>,
        desc: L,
        target: &NodeRef<Target>,
    ) where
        for<'a> L: Lens<'a, Input = Target, Output = Type>,
    {
        self.nodes.add_edge(
            source.idx,
            target.idx,
            (Box::new(desc), |s, t, lens, cx| {
                let source = s.as_any_mut().downcast_mut::<Source>().unwrap();
                let val = source.get(cx);
                let target = t.as_any_mut().downcast_mut::<Target>().unwrap();
                let l = lens.downcast_mut::<L>().unwrap();
                unsafe {
                    ptr::write(l.get_mut(target), val);
                }
            }),
        );
    }

    pub fn update(&mut self, cx: SceneContext) {
        let mut dfs = PostOrderDFS::new(&self.nodes, StartNode::SearchAll);

        while let Some((src, ex, dst)) = dfs.next(&self.nodes) {
            let (n, e) = self.nodes.get_disjoint_mut([src, dst], [ex]);
            let [source, dest] = n.unwrap();
            let [edge] = e.unwrap();
            (edge.1)(source, dest, &mut edge.0, &cx);
        }
    }

    pub fn debug<S: Debug + 'static>(&self, node: NodeIdx) {
        let me = &self.nodes[node];
        let source = me.as_any().downcast_ref::<S>().unwrap();
        println!("{:?}", source);
    }

    #[inline]
    pub fn get_node_mut<N: Node + 'static>(&mut self, node: NodeIdx) -> Option<&mut N> {
        self.nodes[node].as_any_mut().downcast_mut()
    }
}
