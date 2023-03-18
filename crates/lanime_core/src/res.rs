use crate::NodeIdx;

pub enum Res<T> {
    Owned(T),
    Node(NodeIdx),
}
