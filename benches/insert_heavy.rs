use bustle::*;
use conc_map_bench::{
    CHashMapTable, ContrieTable, DashMapExperimentalTable, DashMapV3Table, FlurryTable,
    MutexStdTable, RwLockStdTable,
};

fn main() {
    tracing_subscriber::fmt::init();
    println!("Insert heavy preset:");
    println!("  Read: 10%");
    println!("  Insert: 80%");
    println!("  Update: 10%");
    println!("  Remove: 0%");
    println!("  Upsert: 0%\n");

    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::insert_heavy()).run::<MutexStdTable<u64>>();
    }
    println!("");

    println!("-- RwLockStd");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::insert_heavy()).run::<RwLockStdTable<u64>>();
    }
    println!("");

    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::insert_heavy()).run::<CHashMapTable<u64>>();
    }
    println!("");

    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::insert_heavy()).run::<ContrieTable<u64>>();
    }
    println!("");

    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::insert_heavy()).run::<FlurryTable<u64>>();
    }
    println!("");

    println!("-- DashMapV3");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::insert_heavy()).run::<DashMapV3Table<u64>>();
    }
    println!("");

    println!("-- DashMapExperimental");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::insert_heavy()).run::<DashMapExperimentalTable<u64>>();
    }
    println!("");
}
