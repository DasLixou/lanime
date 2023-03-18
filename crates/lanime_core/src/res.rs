use crate::{Node, NodeIdx, NodeOutput};

pub struct Resource<T>(pub T);

impl<T> Node for Resource<T> {}
impl<T> NodeOutput<T> for Resource<T> {}

pub enum Res<T> {
    Value(T),
    Node(NodeIdx),
}

pub trait IntoRes<T> {
    fn res(self) -> Res<T>;
}
