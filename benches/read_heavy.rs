use bustle::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use fxhash::FxBuildHasher;
use chashmap::CHashMap;
use contrie::ConMap;

#[derive(Clone)]
struct MutexStdTable<K>(Arc<Mutex<HashMap<K, u32, FxBuildHasher>>>);

impl<K> Collection for MutexStdTable<K>
where
    K: Send + From<u64> + Copy + 'static + std::hash::Hash + Eq,
{
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(Mutex::new(HashMap::with_capacity_and_hasher(
            capacity,
            FxBuildHasher::default()
        ))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for MutexStdTable<K>
where
    K: Send + From<u64> + Copy + 'static + std::hash::Hash + Eq,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.lock().unwrap().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.lock().unwrap().insert(*key, 0).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.lock().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut map = self.0.lock().unwrap();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}

#[derive(Clone)]
struct RwLockStdTable<K>(Arc<RwLock<HashMap<K, u32, FxBuildHasher>>>);

impl<K> Collection for RwLockStdTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Handle = Self;
    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(HashMap::with_capacity_and_hasher(
            capacity,
            FxBuildHasher::default()
        ))))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for RwLockStdTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().unwrap().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().insert(*key, 0).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        let mut map = self.0.write().unwrap();
        map.get_mut(key).map(|v| *v += 1).is_some()
    }
}

#[derive(Clone)]
struct CHashMapTable<K>(Arc<CHashMap<K, u32>>);

impl<K> Collection for CHashMapTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
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
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, 0).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.get_mut(key).map(|mut r| { *r += 1; }).is_some()
    }
}

#[derive(Clone)]
struct ContrieTable<K: Eq + std::hash::Hash + 'static>(Arc<ConMap<K, Mutex<u32>>>);

impl<K> Collection for ContrieTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Handle = Self;
    fn with_capacity(_: usize) -> Self {
        Self(Arc::new(ConMap::new()))
    }

    fn pin(&self) -> Self::Handle {
        self.clone()
    }
}

impl<K> CollectionHandle for ContrieTable<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + std::hash::Hash + Eq + std::fmt::Debug,
{
    type Key = K;

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.insert(*key, Mutex::new(0)).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.get(key).map(|e| { *e.value().lock().unwrap() += 1; }).is_some()
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<MutexStdTable<u64>>();
    }
    println!("");

    println!("-- RwLockStd");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<RwLockStdTable<u64>>();
    }
    println!("");

    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<CHashMapTable<u64>>();
    }
    println!("");

    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<ContrieTable<u64>>();
    }
    println!("");
}
