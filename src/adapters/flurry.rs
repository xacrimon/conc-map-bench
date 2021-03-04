use std::hash::BuildHasher;
use std::sync::Arc;

use bustle::*;

#[derive(Clone)]
pub struct FlurryTable<H>(Arc<flurry::HashMap<u64, u32, H>>);

pub struct FlurryHandle<H: 'static>(flurry::HashMapRef<'static, u64, u32, H>);

impl<H> Collection for FlurryTable<H>
where
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = FlurryHandle<H>;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(flurry::HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        unsafe { std::mem::transmute(self.0.pin()) }
    }
}

impl<H> CollectionHandle for FlurryHandle<H>
where
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.compute_if_present(key, |_, v| Some(v + 1)).is_some()
    }
}
