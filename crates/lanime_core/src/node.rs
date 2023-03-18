use std::marker::PhantomData;

use crate::{IntoRes, NodeIdx, Res};

pub trait Node {}

pub type BoxedNode = Box<dyn Node>;

pub trait NodeOutput<T>: Node {}

#[derive(Clone, Copy)]
pub struct NodeRef<N: Node> {
    pub(crate) idx: NodeIdx,
    pub(crate) phantom: PhantomData<N>,
}

impl<N: Node, T> IntoRes<T> for NodeRef<N>
where
    N: NodeOutput<T>,
{
    #[inline]
    fn res(self) -> Res<T> {
        Res::Node(self.idx)
    }
}
