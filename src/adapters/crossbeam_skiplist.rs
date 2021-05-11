use std::sync::Arc;

use bustle::*;
use crossbeam_skiplist::SkipMap;
use parking_lot::Mutex;

use super::Value;

pub struct CrossbeamSkipMapTable<K>(Arc<SkipMap<K, Mutex<Value>>>);

impl<K> Collection for CrossbeamSkipMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + Ord + 'static,
{
    type Handle = Self;

    fn with_capacity(_: usize) -> Self {
        Self(Arc::new(SkipMap::new()))
    }

    fn pin(&self) -> Self::Handle {
        Self(self.0.clone())
    }
}

impl<K> CollectionHandle for CrossbeamSkipMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let map = &mut self.0;
        let prev = map.get(key).is_none();
        map.insert(*key, Mutex::new(0));
        prev
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
