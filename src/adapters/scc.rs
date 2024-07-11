use bustle::*;
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use super::Value;

#[derive(Clone)]
pub struct SccMapTable<K: Eq + Hash + Sync + 'static, H: BuildHasher + 'static>(
    Arc<scc::HashMap<K, Value, H>>,
);

impl<K, H> Collection for SccMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(scc::HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for SccMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read(key, |_, _| ()).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.update(key, |_, v| *v += 1).is_some()
    }
}

#[derive(Clone)]
pub struct SccIndexTable<K: Clone + Eq + Hash + Sync + 'static, H: BuildHasher + 'static>(
    Arc<scc::HashIndex<K, Value, H>>,
);

impl<K, H> Collection for SccIndexTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(scc::HashIndex::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for SccIndexTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + Hash + Ord + 'static,
    H: BuildHasher + Default + Send + Sync + Clone + 'static,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.peek_with(key, |_, _| ()).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_ok()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key)
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        if let scc::hash_index::Entry::Occupied(mut o) = self.0.entry(*key) {
            unsafe {
                let val = o.get_mut();
                *val += 1;
            }
            true
        } else {
            false
        }
    }
}
