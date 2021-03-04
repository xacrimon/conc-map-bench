use std::collections::hash_map::RandomState;
use std::{fmt::Debug, thread::sleep, time::Duration};

use bustle::*;
use fxhash::FxBuildHasher;

use self::adapters::{
    DashMapTable, EvmapTable, FlurryTable, RwLockBTreeMapTable, RwLockStdHashMapTable,
};

mod adapters;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn gc_cycle() {
    sleep(Duration::from_millis(1000));
    let mut new_guard = crossbeam_epoch::pin();
    new_guard.flush();
    for _ in 0..32 {
        new_guard.repin();
    }
    let mut old_guard = crossbeam_epoch_old::pin();
    old_guard.flush();

    for _ in 0..32 {
        old_guard.repin();
    }
}

fn read_heavy(n: usize) -> Workload {
    let mix = Mix {
        read: 98,
        insert: 1,
        remove: 1,
        update: 0,
        upsert: 0,
    };

    *Workload::new(n, mix)
        .initial_capacity_log2(24)
        .prefill_fraction(0.8)
        .operations(0.2)
}

fn rg_mix() -> Mix {
    Mix {
        read: 5,
        insert: 80,
        remove: 5,
        update: 10,
        upsert: 0,
    }
}

fn rapid_grow(n: usize) -> Workload {
    *Workload::new(n, rg_mix())
        .initial_capacity_log2(24)
        .prefill_fraction(0.0)
        .operations(1.5)
}

fn ex_mix() -> Mix {
    Mix {
        read: 10,
        insert: 40,
        remove: 40,
        update: 10,
        upsert: 0,
    }
}

fn exchange(n: usize) -> Workload {
    *Workload::new(n, ex_mix())
        .initial_capacity_log2(24)
        .prefill_fraction(0.8)
        .operations(1.5)
}

/*
fn exchange_task() {
    println!("== exchange");
    println!("-- RwLockStd");
    for n in 1..=num_cpus::get() {
        exchange(n).run::<RwLockStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        exchange(n).run::<MutexStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        exchange(n).run::<CHashMapTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        exchange(n).run::<FlurryTable>();
        gc_cycle();
    }
    println!("");
    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        exchange(n).run::<ContrieTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- DashMap");
    for n in 1..=num_cpus::get() {
        exchange(n).run::<DashMapTable<u64>>();
        gc_cycle();
    }
    println!("==");
}
*/

fn run<C, M, R>(name: &str, make_workload: M, range: R)
where
    C: Collection,
    <C::Handle as CollectionHandle>::Key: Send + Debug,
    M: Fn(usize) -> Workload,
    R: Iterator<Item = usize>,
{
    println!("-- {}", name);
    for n in range {
        let m = make_workload(n).run_silently::<C>();
        eprintln!(
            "total_ops={}\tthreads={}\tspent={:.1?}\tlatency={:?}\tthroughput={:.0}op/s",
            m.total_ops, m.threads, m.spent, m.latency, m.throughput,
        );
        gc_cycle();
    }
    println!();
}

fn cache_task() {
    //let cpus = num_cpus::get();
    let range = || 1..=12;

    println!("== cache");
    // TODO: add `CHashMap` and so on.
    run::<RwLockBTreeMapTable<u64>, _, _>("RwLock<BTreeMap>", read_heavy, range());
    run::<RwLockStdHashMapTable<u64, RandomState>, _, _>("RwLock<StdHashMap>", read_heavy, range());
    run::<RwLockStdHashMapTable<u64, FxBuildHasher>, _, _>(
        "RwLock<FxHashMap>",
        read_heavy,
        range(),
    );
    run::<FlurryTable<RandomState>, _, _>("FlurryTable", read_heavy, range());
    run::<FlurryTable<FxBuildHasher>, _, _>("FxFlurryTable", read_heavy, range());
    run::<DashMapTable<u64, RandomState>, _, _>("DashMapTable", read_heavy, range());
    run::<DashMapTable<u64, FxBuildHasher>, _, _>("FxDashMapTable", read_heavy, range());
    run::<EvmapTable<u64, RandomState>, _, _>("EvmapTable", read_heavy, range());
    run::<EvmapTable<u64, FxBuildHasher>, _, _>("FxEvmapTable", read_heavy, range());
}

/*
fn rapid_grow_task() {
    println!("== rapid grow");
    println!("-- RwLockStd");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<RwLockStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<MutexStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<CHashMapTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<FlurryTable>();
        gc_cycle();
    }
    println!("");
    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<ContrieTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- DashMap");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<DashMapTable<u64>>();
        gc_cycle();
    }
    println!("==");
}
*/

fn main() {
    tracing_subscriber::fmt::init();

    cache_task();
    //exchange_task();
    //rapid_grow_task();
}
