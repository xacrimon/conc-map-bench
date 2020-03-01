use bustle::*;
use conc_map_bench::{
    CHashMapTable, ContrieTable, DashMapExperimentalTable, DashMapV3Table, FlurryTable,
    MutexStdTable, RwLockStdTable,
};

fn main() {
    tracing_subscriber::fmt::init();
    println!("Update heavy preset:");
    println!("  Read: 35%");
    println!("  Insert: 5%");
    println!("  Update: 50%");
    println!("  Remove: 5%");
    println!("  Upsert: 5%\n");

    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::update_heavy()).run::<MutexStdTable<u64>>();
    }
    println!("");

    println!("-- RwLockStd");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::update_heavy()).run::<RwLockStdTable<u64>>();
    }
    println!("");

    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::update_heavy()).run::<CHashMapTable<u64>>();
    }
    println!("");

    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::update_heavy()).run::<ContrieTable<u64>>();
    }
    println!("");

    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::update_heavy()).run::<FlurryTable<u64>>();
    }
    println!("");

    println!("-- DashMapV3");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::update_heavy()).run::<DashMapV3Table<u64>>();
    }
    println!("");

    println!("-- DashMapExperimental");
    for n in 1..=num_cpus::get() {
        Workload::new(n, Mix::update_heavy()).run::<DashMapExperimentalTable<u64>>();
    }
    println!("");
}
