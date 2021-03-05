use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;

use super::Value;

#[derive(Clone)]
pub struct FlurryTable<K, H>(Arc<flurry::HashMap<K, Value, H>>);

pub struct FlurryHandle<K: 'static, H: 'static>(flurry::HashMapRef<'static, K, Value, H>);

impl<K, H> Collection for FlurryTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = FlurryHandle<K, H>;

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

impl<K, H> CollectionHandle for FlurryHandle<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

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
