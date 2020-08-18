mod adapters;

use adapters::{DashMapTable, ShardTable};
use bustle::*;

fn read_heavy(n: usize) -> Workload {
    *Workload::new(n, Mix::read_heavy())
        .initial_capacity_log2(24)
        .prefill_fraction(0.8)
        .operations(1.5)
}

fn cache_task() {
    println!("-- DashMap");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<DashMapTable<u64>>();

    }
    println!("");
    println!("-- Shard");
    for n in 1..=num_cpus::get() {
        read_heavy(n).run::<ShardTable<u64>>();

    }
}

fn main() {
    tracing_subscriber::fmt::init();

    cache_task();
}
