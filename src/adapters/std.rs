use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;
use parking_lot::RwLock;
use std::sync::RwLock as StdRwLock;

use super::Value;

#[derive(Clone)]
pub struct ParkingLotRwLockStdHashMapTable<K, H>(Arc<RwLock<HashMap<K, Value, H>>>);

impl<K, H> Collection for ParkingLotRwLockStdHashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        ))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for ParkingLotRwLockStdHashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut map = self.0.write();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}

#[derive(Clone)]
pub struct StdRwLockStdHashMapTable<K, H>(Arc<StdRwLock<HashMap<K, Value, H>>>);

impl<K, H> Collection for StdRwLockStdHashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(StdRwLock::new(HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        ))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for StdRwLockStdHashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().unwrap().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .write()
            .unwrap()
            .get_mut(key)
            .map(|v| *v += 1)
            .is_some()
    }
}
