use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;
use parking_lot::Mutex;

use super::Value;

#[derive(Clone)]
pub struct EvmapTable<K: Hash + Eq + Clone, H: BuildHasher + Clone> {
    rd: Arc<Mutex<evmap::ReadHandle<K, Value, (), H>>>,
    wr: Arc<Mutex<evmap::WriteHandle<K, Value, (), H>>>,
}

impl<K, H> Collection for EvmapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = EvmapTableHandle<K, H>;

    fn with_capacity(capacity: usize) -> Self {
        let (rd, wr) = evmap::Options::default()
            .with_hasher(H::default())
            .with_capacity(capacity)
            .construct();

        Self {
            rd: Arc::new(Mutex::new(rd)),
            wr: Arc::new(Mutex::new(wr)),
        }
    }

    fn pin(&self) -> Self::Handle {
        EvmapTableHandle {
            rd: self.rd.lock().clone(),
            wr: self.wr.clone(),
        }
    }
}

pub struct EvmapTableHandle<K: Hash + Eq + Clone, H: BuildHasher + Clone> {
    rd: evmap::ReadHandle<K, u32, (), H>,
    wr: Arc<Mutex<evmap::WriteHandle<K, u32, (), H>>>,
}

impl<K, H> CollectionHandle for EvmapTableHandle<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.rd.get_one(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        let prev = self.rd.get_one(key).is_none();
        self.wr.lock().insert(*key, 0).refresh();
        prev
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        let prev = self.rd.get_one(key).is_some();
        self.wr.lock().empty(*key).refresh();
        prev
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let val = match self.rd.get_one(key) {
            Some(val) => *val + 1,
            None => return false,
        };

        let prev = self.rd.get_one(key).is_some();
        self.wr.lock().update(*key, val).refresh();
        prev
    }
}
