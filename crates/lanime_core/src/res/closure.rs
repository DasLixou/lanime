use crate::{Node, NodeResult, SceneContext};

pub struct ClosureResource<T>(pub fn(&SceneContext) -> T);

impl<T: 'static> Node for ClosureResource<T> {}
impl<T: 'static> NodeResult<T> for ClosureResource<T> {
    #[inline]
    fn get(&mut self, cx: &SceneContext) -> T {
        (self.0)(cx)
    }
}
