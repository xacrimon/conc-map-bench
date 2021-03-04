use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use bustle::*;
use chashmap::CHashMap;
use contrie::ConMap;
use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};

#[derive(Clone)]
pub struct RwLockBTreeMapTable<K>(Arc<RwLock<BTreeMap<K, u32>>>);

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

#[derive(Clone)]
pub struct RwLockStdHashMapTable<K, H>(Arc<RwLock<HashMap<K, u32, H>>>);

impl<K, H> Collection for RwLockStdHashMapTable<K, H>
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

impl<K, H> CollectionHandle for RwLockStdHashMapTable<K, H>
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
pub struct MutexStdHashMapTable<K, H>(Arc<Mutex<HashMap<K, u32, H>>>);

impl<K, H> Collection for MutexStdHashMapTable<K, H>
where
    K: Send + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(Mutex::new(HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        ))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for MutexStdHashMapTable<K, H>
where
    K: Send + From<u64> + Copy + 'static + Hash + Eq,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.lock().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.lock().insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.lock().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut map = self.0.lock();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}

#[derive(Clone)]
pub struct CHashMapTable<K>(Arc<CHashMap<K, u32>>);

impl<K> Collection for CHashMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(CHashMap::with_capacity(capacity)))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for CHashMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0
            .get_mut(key)
            .map(|mut r| {
                *r += 1;
            })
            .is_some()
    }
}

#[derive(Clone)]
pub struct ContrieTable<K: Eq + Hash + 'static, H>(Arc<ConMap<K, Mutex<u32>, H>>);

impl<K, H> Collection for ContrieTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
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
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
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

#[derive(Clone)]
pub struct FlurryTable<H>(Arc<flurry::HashMap<u64, u32, H>>);

pub struct FlurryHandle<H: 'static>(flurry::HashMapRef<'static, u64, u32, H>);

impl<H> Collection for FlurryTable<H>
where
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = FlurryHandle<H>;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(flurry::HashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        unsafe { std::mem::transmute(self.0.pin()) }
    }
}

impl<H> CollectionHandle for FlurryHandle<H>
where
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = u64;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_none()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.compute_if_present(key, |_, v| Some(v + 1)).is_some()
    }
}

#[derive(Clone)]
pub struct DashMapTable<K, H>(Arc<DashMap<K, u32, H>>);

impl<K, H> Collection for DashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Handle = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(DashMap::with_capacity_and_hasher(
            capacity,
            H::default(),
        )))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K, H> CollectionHandle for DashMapTable<K, H>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
    H: BuildHasher + Default + Send + Sync + 'static + Clone,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.extract(key, |_, _| ()).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        !self.0.insert(*key, 0)
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key)
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.update(key, |_, v| v + 1)
    }
}

#[derive(Clone)]
pub struct EvmapTable<K: Hash + Eq + Clone, H: BuildHasher + Clone> {
    rd: Arc<Mutex<evmap::ReadHandle<K, u32, (), H>>>,
    wr: Arc<Mutex<evmap::WriteHandle<K, u32, (), H>>>,
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
