use std::hash::{BuildHasher, Hash};
use std::{fmt::Debug, sync::Arc};

use bustle::*;
use contrie::ConMap;
use parking_lot::Mutex;

use super::Value;

#[derive(Clone)]
pub struct ContrieTable<K: Eq + Hash + 'static, H>(Arc<ConMap<K, Mutex<Value>, H>>);

impl<K, H> Collection for ContrieTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(_: usize) -> Self {
        Self(Arc::new(ConMap::with_hasher(H::default())))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for ContrieTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, Mutex::new(0)).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .get(key)
            .map(|e| {
                *e.value().lock() += 1;
            })
            .is_some()
    }
}
