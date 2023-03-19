use crate::{Node, NodeResult, SceneContext};

pub struct StaticResource<T: Clone>(pub T);

impl<T: Clone + 'static> Node for StaticResource<T> {}
impl<T: Clone + 'static> NodeResult<T> for StaticResource<T> {
    #[inline]
    fn get(&mut self, _cx: &SceneContext) -> T {
        self.0.clone()
    }
}
