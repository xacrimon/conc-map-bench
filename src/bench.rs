use std::collections::hash_map::RandomState;
use std::{fmt::Debug, io, thread::sleep, time::Duration};

use bustle::*;
use fxhash::FxBuildHasher;
use structopt::StructOpt;

use crate::{adapters::*, record::Record, workloads};

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long)]
    pub workload: workloads::WorkloadKind,
    #[structopt(short, long, default_value = "1")]
    pub operations: f64,
    #[structopt(long)]
    pub threads: Option<Vec<u32>>,
    #[structopt(long)]
    pub use_std_hasher: bool,
    #[structopt(long, default_value = "2000")]
    pub gc_sleep_ms: u64,
    #[structopt(long)]
    pub skip: Option<Vec<String>>, // TODO: use just `Vec<String>`.
    #[structopt(long, default_value = "5000")]
    pub latency_limit_ns: u64,
    #[structopt(long)]
    pub csv: bool,
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

type Handler = Box<dyn FnMut(&str, u32, &Measurement)>;

fn case<C>(name: &str, options: &Options, handler: &mut Handler)
where
    C: Collection,
    <C::Handle as CollectionHandle>::Key: Send + Debug,
{
    if options
        .skip
        .as_ref()
        .and_then(|s| s.iter().find(|s| s == &name))
        .is_some()
    {
        println!("-- {} [skipped]", name);
        return;
    } else {
        println!("-- {}", name);
    }

    let threads = options
        .threads
        .as_ref()
        .cloned()
        .unwrap_or_else(|| (1..(num_cpus::get() * 3 / 2) as u32).collect());

    for n in &threads {
        let m = workloads::create(options, *n).run_silently::<C>();
        handler(name, *n, &m);

        if m.latency.as_nanos() > options.latency_limit_ns.into() {
            println!("too long, skipped");
            break;
        }
        gc_cycle(options);
    }
    println!();
}

fn run(options: &Options, h: &mut Handler) {
    // TODO: case::<CrossbeamSkipMapTable<u64>>("CrossbeamSkipMap", options, h);
    case::<RwLockBTreeMapTable<u64>>("RwLock<BTreeMap>", options, h);

    if options.use_std_hasher {
        case::<RwLockStdHashMapTable<u64, RandomState>>("RwLock<StdHashMap>", options, h);
        case::<DashMapTable<u64, RandomState>>("DashMap", options, h);
        case::<FlurryTable<u64, RandomState>>("Flurry", options, h);
        case::<EvmapTable<u64, RandomState>>("Evmap", options, h);
        case::<CHashMapTable<u64>>("CHashMap", options, h);
    } else {
        case::<RwLockStdHashMapTable<u64, FxBuildHasher>>("RwLock<FxHashMap>", options, h);
        case::<DashMapTable<u64, FxBuildHasher>>("FxDashMap", options, h);
        case::<FlurryTable<u64, FxBuildHasher>>("FxFlurry", options, h);
        case::<EvmapTable<u64, FxBuildHasher>>("FxEvmap", options, h);
    }
}

pub fn bench(options: &Options) {
    println!("== {:?}", options.workload);

    let mut handler = if options.csv {
        let mut wr = csv::Writer::from_writer(io::stderr());

        Box::new(move |name: &str, n, m: &Measurement| {
            wr.serialize(Record {
                name: name.into(),
                total_ops: m.total_ops,
                threads: n,
                spent: m.spent,
                throughput: m.throughput,
                latency: m.latency,
            })
            .expect("cannot serialize");
            wr.flush().expect("cannot flush");
        }) as Handler
    } else {
        Box::new(|_: &str, n, m: &Measurement| {
            eprintln!(
                "total_ops={}\tthreads={}\tspent={:.1?}\tlatency={:?}\tthroughput={:.0}op/s",
                m.total_ops, n, m.spent, m.latency, m.throughput,
            );
        }) as Handler
    };

    run(&options, &mut handler);
}
