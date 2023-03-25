use crate::clip_metadata::ClipMetadata;

pub trait Clip<T> {
    fn get_at(&mut self, relative_time: usize, meta: ClipMetadata) -> T;
}
