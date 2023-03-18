use crate::Node;

pub struct Resource<T>(pub T);

impl<T> Node for Resource<T> {}
