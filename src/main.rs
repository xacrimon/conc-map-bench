use std::collections::hash_map::RandomState;
use std::{fmt::Debug, thread::sleep, time::Duration};

use bustle::*;
use fxhash::FxBuildHasher;
use structopt::StructOpt;

use self::adapters::*;

mod adapters;
mod workloads;

#[derive(Debug, StructOpt)]
struct Options {
    workload: workloads::WorkloadKind,
    #[structopt(short, long, default_value = "0.1")]
    operations: f64,
    #[structopt(long)]
    threads: Option<Vec<usize>>,
    #[structopt(long)]
    use_std_hasher: bool,
    #[structopt(default_value = "2000")]
    gc_sleep_ms: u64,
}

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn gc_cycle(options: &Options) {
    sleep(Duration::from_millis(options.gc_sleep_ms));
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

fn case<C>(name: &str, options: &Options)
where
    C: Collection,
    <C::Handle as CollectionHandle>::Key: Send + Debug,
{
    println!("-- {}", name);

    let threads = options
        .threads
        .as_ref()
        .cloned()
        .unwrap_or_else(|| (1..(num_cpus::get() * 2 / 3)).collect());

    for n in &threads {
        let m = workloads::create(options, *n).run_silently::<C>();

        eprintln!(
            "total_ops={}\tthreads={}\tspent={:.1?}\tlatency={:?}\tthroughput={:.0}op/s",
            m.total_ops, n, m.spent, m.latency, m.throughput,
        );
        gc_cycle(options);
    }
    println!();
}

fn main() {
    tracing_subscriber::fmt::init();

    let options = &Options::from_args();
    println!("== {:?}", options.workload);

    case::<RwLockBTreeMapTable<u64>>("RwLock<BTreeMap>", options);
    // TODO: case::<CrossbeamSkipMapTable<u64>>("CrossbeamSkipMap", options);

    if options.use_std_hasher {
        case::<RwLockStdHashMapTable<u64, RandomState>>("RwLock<StdHashMap>", options);
        case::<DashMapTable<u64, RandomState>>("DashMap", options);
        case::<FlurryTable<RandomState>>("Flurry", options);
        case::<EvmapTable<u64, RandomState>>("Evmap", options);
        case::<CHashMapTable<u64>>("CHashMap", options);
    } else {
        case::<RwLockStdHashMapTable<u64, FxBuildHasher>>("RwLock<FxHashMap>", options);
        case::<DashMapTable<u64, FxBuildHasher>>("FxDashMap", options);
        case::<FlurryTable<FxBuildHasher>>("FxFlurry", options);
        case::<EvmapTable<u64, FxBuildHasher>>("FxEvmap", options);
    }
}
