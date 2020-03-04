mod adapters;

use adapters::{CHashMapTable, ContrieTable, DashMapV3Table, FlurryTable, MutexStdTable};
use bustle::*;
use std::thread::sleep;
use std::time::Duration;

const PREFILL: f64 = 0.5;
const OPS: f64 = 1.0;

fn pause() {
    sleep(Duration::from_secs(1));
}

fn read_heavy(n: usize) -> Workload {
    *Workload::new(n, Mix::read_heavy()).prefill_fraction(PREFILL).operations(OPS)
}

fn main() {
    tracing_subscriber::fmt::init();

    println!("-- MutexStd");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<MutexStdTable<u64>>();
    }
    println!("");
    pause();
    println!("-- CHashMap");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<CHashMapTable<u64>>();
    }
    println!("");
    pause();
    println!("-- Flurry");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<FlurryTable<u64>>();
    }
    println!("");
    pause();
    println!("-- Contrie");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<ContrieTable<u64>>();
    }
    println!("");
    pause();
    println!("-- DashMapV3");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<DashMapV3Table<u64>>();
    }
}
