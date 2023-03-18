use std::marker::PhantomData;

use crate::NodeIdx;

pub trait Node {}

pub type BoxedNode = Box<dyn Node>;

pub trait NodeResult<T>: Node {}

#[derive(Clone, Copy)]
pub struct NodeRef<N: Node> {
    pub(crate) idx: NodeIdx,
    pub(crate) phantom: PhantomData<N>,
}

mod sealed {
    pub trait Sealed {}
}

pub trait NodeOutput<T>: sealed::Sealed {}

impl<N: Node> sealed::Sealed for NodeRef<N> {}
impl<N: Node, T> NodeOutput<T> for NodeRef<N> where N: NodeResult<T> {}
