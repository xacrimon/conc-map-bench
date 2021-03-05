use std::marker::PhantomData;

use bustle::*;
use crossbeam_skiplist::SkipMap;

use super::Value;

pub struct CrossbeamSkipMapTable<K>(PhantomData<K>);

impl<K> Collection for CrossbeamSkipMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + Ord + 'static,
{
    type Handle = CrossbeamSkipMapHandle<K>;

    fn with_capacity(_: usize) -> Self {
        Self(PhantomData)
    }

    fn pin(&self) -> Self::Handle {
        CrossbeamSkipMapHandle(SkipMap::new())
    }
}

pub struct CrossbeamSkipMapHandle<K>(SkipMap<K, Value>);

impl<K> CollectionHandle for CrossbeamSkipMapHandle<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let prev = self.0.get(key).is_none();
        self.0.insert(*key, 0);
        prev
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let entry = match self.0.get(key) {
            Some(entry) => entry,
            None => return false,
        };

        self.0.insert(*key, entry.value() + 1);
        true
    }
}
