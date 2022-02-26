use std::hash::{BuildHasher, Hash};

use bustle::*;
use seize::Collector;

use super::Value;

const BATCH_SIZE: usize = 2000;

#[derive(Clone)]
pub struct FlurryTable<K: 'static, H: 'static>(&'static flurry::HashMap<K, Value, H>);

impl<K, H> Collection for FlurryTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = FlurryHandle<K, H>;

    fn with_capacity(capacity: usize) -> Self {
        Self(Box::leak(Box::new(
            flurry::HashMap::with_capacity_and_hasher(capacity, H::default()).with_collector(
                Collector::new()
                    .epoch_frequency(None)
                    .batch_size(BATCH_SIZE),
            ),
        )))
    }

    fn pin(&self) -> Self::Handle {
        FlurryHandle(self.0)
    }
}

pub struct FlurryHandle<K: 'static, H: 'static>(&'static flurry::HashMap<K, Value, H>);

impl<K, H> CollectionHandle for FlurryHandle<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.pin().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.pin().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.pin().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .pin()
            .compute_if_present(key, |_, v| Some(v + 1))
            .is_some()
    }
}
