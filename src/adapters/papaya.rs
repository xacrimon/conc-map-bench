use bustle::*;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;

const BATCH_SIZE: usize = 2000;

#[derive(Clone)]
pub struct PapayaTable<K: 'static, H: 'static>(Arc<papaya::HashMap<K, Value, H>>);

impl<K, H> Collection for PapayaTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Ord,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        let map = papaya::HashMap::builder()
            .capacity(capacity)
            .hasher(H::default())
            .collector(
                papaya::Collector::new()
                    .epoch_frequency(None)
                    .batch_size(BATCH_SIZE),
            )
            .build();

        Self(Arc::new(map))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for PapayaTable<K, H>
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
        self.0.pin().update(*key, |v| v + 1).is_some()
    }
}
