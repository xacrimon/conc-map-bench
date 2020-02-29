use bustle::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use fxhash::FxBuildHasher;
use chashmap::CHashMap;

#[derive(Clone)]
struct MutexStdTable<K>(Arc<Mutex<HashMap<K, (), FxBuildHasher>>>);

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
        self.0.lock().unwrap().insert(*key, ()).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.lock().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        use std::collections::hash_map::Entry;
        let mut map = self.0.lock().unwrap();
        if let Entry::Occupied(mut e) = map.entry(*key) {
            e.insert(());
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
struct RwLockStdTable<K>(Arc<RwLock<HashMap<K, (), FxBuildHasher>>>);

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
        self.0.write().unwrap().insert(*key, ()).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        use std::collections::hash_map::Entry;
        let mut map = self.0.write().unwrap();
        if let Entry::Occupied(mut e) = map.entry(*key) {
            e.insert(());
            true
        } else {
            false
        }
    }
}

#[derive(Clone)]
struct CHashMapTable<K>(Arc<CHashMap<K, ()>>);

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
        self.0.insert(*key, ()).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        self.0.get_mut(key).map(|mut r| *r = ()).is_some()
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    println!("-- MutexStd\n");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<MutexStdTable<u64>>();
    }
    println!("");

    println!("-- RwLockStd\n");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<RwLockStdTable<u64>>();
    }
    println!("");

    println!("-- CHashMap\n");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<CHashMapTable<u64>>();
    }
    println!("");
}
