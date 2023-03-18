use crate::{Node, NodeResult};

pub struct Resource<T: Clone>(pub T);

impl<T: Clone + 'static> Node for Resource<T> {}
impl<T: Clone + 'static> NodeResult<T> for Resource<T> {
    fn get(&mut self) -> T {
        self.0.clone()
    }
}
