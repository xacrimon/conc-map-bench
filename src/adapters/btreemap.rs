use std::collections::BTreeMap;
use std::sync::Arc;

use bustle::*;
use parking_lot::RwLock;

use super::Value;

#[derive(Clone)]
pub struct RwLockBTreeMapTable<K>(Arc<RwLock<BTreeMap<K, Value>>>);

impl<K> Collection for RwLockBTreeMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Handle = Self;

    fn with_capacity(_: usize) -> Self {
        Self(Arc::new(RwLock::new(BTreeMap::new())))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for RwLockBTreeMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
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
