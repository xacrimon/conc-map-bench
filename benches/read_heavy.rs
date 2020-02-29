use bustle::*;
use conc_map_bench::{CHashMapTable, ContrieTable, FlurryTable, MutexStdTable, RwLockStdTable, DashMapV3Table};

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

    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<FlurryTable<u64>>();
    }
    println!("");

    println!("-- DashMapV3");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::read_heavy()).run::<DashMapV3Table<u64>>();
    }
    println!("");
}
