use crate::{Node, NodeOutput};

pub struct Resource<T>(pub T);

impl<T> Node for Resource<T> {}

impl<T> NodeOutput<T> for Resource<T> {}
