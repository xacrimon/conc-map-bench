mod adapters;

use adapters::{
    CHashMapTable, ContrieTable, DashMapExperimentalTable, DashMapV3Table, FlurryTable,
    MutexStdTable,
};
use bustle::*;
use std::thread::sleep;
use std::time::Duration;

const CAP: u8 = 25;
const PREFILL: f64 = 0.6;
const OPS: f64 = 1.5;

fn pause() {
    sleep(Duration::from_millis(200));
}

fn read_heavy(n: usize) -> Workload {
    *Workload::new(n, Mix::read_heavy())
        .initial_capacity_log2(CAP)
        .prefill_fraction(PREFILL)
        .operations(OPS)
}

fn main() {
    tracing_subscriber::fmt::init();

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
        read_heavy(n).run::<FlurryTable<u64>>();
    }
    println!("");
    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<ContrieTable<u64>>();
    }
    println!("");
    println!("-- DashMapV3");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<DashMapV3Table<u64>>();
    }
    println!("");
    println!("-- DashMapExperimental");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<DashMapExperimentalTable<u64>>();
        pause();
    }
}
