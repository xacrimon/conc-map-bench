use std::marker::PhantomData;

use bustle::*;
use crossbeam_skiplist::SkipMap;
use parking_lot::Mutex;

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
        CrossbeamSkipMapHandle(Mutex::new(SkipMap::new()))
    }
}

// FIXME: we use `Mutex` to help us estimate where is a bug (skiplist? burst? adapter?)
pub struct CrossbeamSkipMapHandle<K>(Mutex<SkipMap<K, Mutex<Value>>>);

impl<K> CollectionHandle for CrossbeamSkipMapHandle<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Ord,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.lock().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let map = self.0.lock();
        let prev = map.get(key).is_none();
        map.insert(*key, Mutex::new(0));
        prev
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.lock().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .lock()
            .get(key)
            .map(|e| {
                *e.value().lock() += 1;
            })
            .is_some()
    }
}
