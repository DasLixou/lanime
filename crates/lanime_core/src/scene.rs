use std::{any::Any, fmt::Debug};

use lanime_graph::{Graph, NodeIdx};

use crate::{bindable_field::Lens, BoxedNode, Node, NodeRef, NodeResult};

pub trait IntoNodeIdx {
    fn idx(&self) -> NodeIdx;
}

pub struct SceneDescriptor<'a> {
    pub label: Option<&'a str>,
}

pub struct Scene {
    nodes: Graph<
        BoxedNode,
        (
            Box<dyn Any>,
            fn(&mut BoxedNode, &mut BoxedNode, &mut Box<dyn Any>),
        ),
    >,
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
        'a,
        Type,
        Source: Node + NodeResult<Type> + 'static,
        Target: Node + 'static,
        L: 'static,
    >(
        &mut self,
        source: &NodeRef<Source>,
        desc: L,
        target: &NodeRef<Target>,
    ) where
        L: Lens<'a, Input = Target, Output = Type>,
    {
        self.nodes.add_edge(
            source.idx,
            target.idx,
            (Box::new(desc), |s, t, lens| {
                let source = s.as_any_mut().downcast_mut::<Source>().unwrap();
                let val = source.get();
                let target = t.as_any_mut().downcast_mut::<Target>().unwrap();
                let l = lens.downcast_mut::<L>().unwrap();
                let target = unsafe {
                    let ptr = target as *mut Target;
                    &mut *ptr
                };
                l.update(target, val);
            }),
        );
    }

    pub fn update(&mut self, _node: NodeIdx) {
        /* TODO: Post Order DFS

        let me = cry_helper(&mut self.nodes[node]);
        if let Some(bindings) = self.bindings.get_mut(node) {
            for bind in cry_helper(bindings) {
                self.update(bind.0);
                let source = cry_helper(&mut self.nodes[bind.0]);
                (bind.2)(source, me, &mut bind.1);
            }
        }
         */
    }

    pub fn debug<S: Debug + 'static>(&self, node: NodeIdx) {
        let me = &self.nodes[node];
        let source = me.as_any().downcast_ref::<S>().unwrap();
        println!("{:?}", source);
    }
}
