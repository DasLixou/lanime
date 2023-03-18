use std::{
    any::{Any, TypeId},
    fmt::Debug,
};

use as_any::Downcast;
use slotmap::{new_key_type, HopSlotMap, SecondaryMap};

use crate::{bindable_field::Lens, BoxedNode, Node, NodeRef, NodeResult, Resource};

new_key_type! {
    pub struct NodeIdx;
}

pub trait IntoNodeIdx {
    fn idx(&self) -> NodeIdx;
}

pub struct SceneDescriptor<'a> {
    pub label: Option<&'a str>,
}

pub struct Scene {
    nodes: HopSlotMap<NodeIdx, BoxedNode>,
    bindings: SecondaryMap<
        NodeIdx,
        Vec<(
            NodeIdx,
            Box<dyn Any>,
            fn(&mut BoxedNode, &mut BoxedNode, &mut Box<dyn Any>),
        )>,
    >,
}

impl Scene {
    pub fn new(_descriptor: &SceneDescriptor<'_>) -> Self {
        Self {
            nodes: HopSlotMap::with_key(),
            bindings: SecondaryMap::new(),
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

    pub fn bind<'a, Type, Source: Node + NodeResult<Type>, Target: Node, L: 'static>(
        &mut self,
        source: &NodeRef<Source>,
        desc: L,
        target: &NodeRef<Target>,
    ) where
        L: Lens<'a, Input = Target, Output = Type>,
    {
        self.bindings.entry(target.idx).unwrap().or_default().push((
            source.idx(),
            Box::new(desc),
            |s, t, lens| {
                println!("UGHGH {:?}", TypeId::of::<Source>());
                let source = s.downcast_mut::<Source>().unwrap();
                let val = source.get();
                let target = t.downcast_mut::<Target>().unwrap();
                let l = lens.downcast_mut::<L>().unwrap();
                let target = unsafe {
                    let ptr = target as *mut Target;
                    &mut *ptr
                };
                l.update(target, val);
            },
        ));
    }

    pub fn update(&mut self, node: NodeIdx) {
        let me = cry_helper(&mut self.nodes[node]);
        if let Some(bindings) = self.bindings.get_mut(node) {
            for bind in cry_helper(bindings) {
                self.update(bind.0);
                let source = cry_helper(&mut self.nodes[bind.0]);
                println!("wo problem? {:?}", source.type_id());
                println!("wo problem? {:?}", source.type_name()); // this say's its Resource<f32> but hasn't the TypeId of it (cmp above and below)
                println!("wo problem2? {:?}", TypeId::of::<Resource<f32>>());
                (bind.2)(source, me, &mut bind.1);
            }
        }
    }

    pub fn debug<S: Debug + 'static>(&self, node: NodeIdx) {
        let me = &self.nodes[node];
        let source = me.downcast_ref::<S>().unwrap();
        println!("{:?}", source);
    }
}

fn cry_helper<'a, 'b, T>(x: &'a mut T) -> &'b mut T {
    unsafe { &mut *(x as *mut T) }
}
