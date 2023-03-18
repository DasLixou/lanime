use std::marker::PhantomData;

use crate::{as_any::AsAny, IntoNodeIdx, NodeIdx};

pub trait Node: AsAny {}

pub type BoxedNode = Box<dyn Node>;

pub trait NodeResult<T>: Node {
    fn get(&mut self) -> T;
}

#[derive(Clone, Copy)]
pub struct NodeRef<N: Node> {
    pub(crate) idx: NodeIdx,
    pub(crate) phantom: PhantomData<N>,
}

impl<N: Node> IntoNodeIdx for NodeRef<N> {
    #[inline]
    fn idx(&self) -> NodeIdx {
        self.idx
    }
}

mod sealed {
    pub trait Sealed {}
}

pub trait NodeOutput<T>: sealed::Sealed + IntoNodeIdx {}

impl<N: Node> sealed::Sealed for NodeRef<N> {}
impl<N: Node, T> NodeOutput<T> for NodeRef<N> where N: NodeResult<T> {}
