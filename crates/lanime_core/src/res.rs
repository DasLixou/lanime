use crate::NodeIdx;

pub enum Res<T> {
    Owned(T),
    Node(NodeIdx),
}

pub trait IntoRes<T> {
    fn res(self) -> Res<T>;
}
