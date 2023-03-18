use crate::{Node, NodeResult};

pub struct Resource<T>(pub T);

impl<T> Node for Resource<T> {}
impl<T> NodeResult<T> for Resource<T> {}
