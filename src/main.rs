mod adapters;

use adapters::{
    CHashMapTable, ContrieTable, DashMapTable, FlurryTable,
    MutexStdTable,
};
use bustle::*;
use std::thread::sleep;
use std::time::Duration;

fn pause() {
    sleep(Duration::from_millis(200));
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

fn main() {
    tracing_subscriber::fmt::init();

    println!("== read heavy");
    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<MutexStdTable<u64>>();
    }
    println!("");
    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<CHashMapTable<u64>>();
    }
    println!("");
    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        pause();
        read_heavy(n).run::<FlurryTable>();
    }
    println!("");
    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        pause();
        read_heavy(n).run::<ContrieTable<u64>>();
    }
    println!("");
    println!("-- DashMap");
    for n in 1..=num_cpus::get() {
        pause();
        read_heavy(n).run::<DashMapTable<u64>>();
    }
    println!("==");

    println!("== rapid grow");
    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<MutexStdTable<u64>>();
    }
    println!("");
    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        rapid_grow(n).run::<CHashMapTable<u64>>();
    }
    println!("");
    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        pause();
        rapid_grow(n).run::<FlurryTable>();
    }
    println!("");
    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        pause();
        rapid_grow(n).run::<ContrieTable<u64>>();
    }
    println!("");
    println!("-- DashMap");
    for n in 1..=num_cpus::get() {
        pause();
        rapid_grow(n).run::<DashMapTable<u64>>();
    }
    println!("==");
}
