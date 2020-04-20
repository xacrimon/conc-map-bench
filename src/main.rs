mod adapters;

use adapters::{
    CHashMapTable, ContrieTable, DashMapTable, FlurryTable,
    MutexStdTable,
};
use bustle::*;
use std::thread::sleep;
use std::time::Duration;

fn gc_cycle() {
    sleep(Duration::from_millis(200));
    let mut new_guard = crossbeam_epoch::pin();
    new_guard.flush();
    for _ in 0..8 {
        new_guard.repin();
    }
    let mut old_guard = crossbeam_epoch_old::pin();
    old_guard.flush();

    for _ in 0..8 {
        old_guard.repin();
    }
}

fn read_heavy(n: usize) -> Workload {
    *Workload::new(n, Mix::read_heavy())
        .initial_capacity_log2(24)
        .prefill_fraction(0.8)
        .operations(1.5)
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

fn main() {
    tracing_subscriber::fmt::init();

    println!("== cache");
    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<MutexStdTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<CHashMapTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<FlurryTable>();
        gc_cycle();
    }
    println!("");
    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<ContrieTable<u64>>();
        gc_cycle();
    }
    println!("");
    println!("-- DashMap");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<DashMapTable<u64>>();
        gc_cycle();
    }
    println!("==\n");

    println!("== rapid grow");
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
    println!("==\n");

    println!("== exchange");
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
