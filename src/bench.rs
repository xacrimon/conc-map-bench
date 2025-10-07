use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::iter;
use std::{fmt::Debug, io, thread::sleep, time::Duration};

use bustle::*;
use structopt::StructOpt;

use crate::{adapters::*, record::Record, workloads};

#[derive(Debug)]
pub enum HasherKind {
    Std,
    AHash,
}

fn parse_hasher_kind(hasher: &str) -> Result<HasherKind, &str> {
    match hasher {
        "std" => Ok(HasherKind::Std),
        "ahash" => Ok(HasherKind::AHash),
        _ => Err("invalid hasher, must be one of 'std' or 'ahash'"),
    }
}

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long)]
    pub workload: workloads::WorkloadKind,
    #[structopt(short, long, default_value = "1")]
    pub operations: f64,
    #[structopt(long)]
    pub threads: Option<Vec<u32>>,
    #[structopt(short, long, parse(try_from_str = parse_hasher_kind))]
    pub hasher: HasherKind,
    #[structopt(long, default_value = "2000")]
    pub gc_sleep_ms: u64,
    #[structopt(long)]
    pub skip: Option<Vec<String>>, // TODO: use just `Vec<String>`.
    #[structopt(long)]
    pub csv: bool,
    #[structopt(long)]
    pub csv_no_headers: bool,

    // Custom workload parameters
    #[structopt(long, help = "Read percentage (0-100)")]
    pub read: Option<u8>,
    #[structopt(long, help = "Insert percentage (0-100)")]
    pub insert: Option<u8>,
    #[structopt(long, help = "Remove percentage (0-100)")]
    pub remove: Option<u8>,
    #[structopt(long, help = "Update percentage (0-100)")]
    pub update: Option<u8>,
    #[structopt(long, help = "Upsert percentage (0-100)")]
    pub upsert: Option<u8>,
    #[structopt(long, help = "Initial capacity log2", default_value = "25")]
    pub capacity_log2: u8,
    #[structopt(long, help = "Prefill fraction (0.0-1.0)", default_value = "0.75")]
    pub prefill: f64,
}

fn gc_cycle(options: &Options) {
    sleep(Duration::from_millis(options.gc_sleep_ms));
    let mut new_guard = crossbeam_epoch::pin();
    new_guard.flush();
    for _ in 0..32 {
        new_guard.repin();
    }
}

fn validate_workload(options: &Options) -> Result<(), &str> {
    if matches!(options.workload, workloads::WorkloadKind::Custom) {
        let read = options.read.unwrap_or(0);
        let insert = options.insert.unwrap_or(0);
        let remove = options.remove.unwrap_or(0);
        let update = options.update.unwrap_or(0);
        let upsert = options.upsert.unwrap_or(0);
        let prefill = options.prefill;

        let total = read + insert + remove + update + upsert;

        if total != 100 {
            return Err("operations percentages must sum to 100");
        }
        if prefill < 0.0 || prefill > 1.0 {
            return Err("prefill must be between 0.0 and 1.0");
        }
    }
    Ok(())
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

    let gen_threads = || {
        let n = num_cpus::get();

        match n {
            0..=10 => (1..=n as u32).collect(),
            11..=16 => iter::once(1)
                .chain((0..=n as u32).step_by(2).skip(1))
                .collect(),
            _ => iter::once(1)
                .chain((0..=n as u32).step_by(4).skip(1))
                .collect(),
        }
    };

    let threads = options
        .threads
        .as_ref()
        .cloned()
        .unwrap_or_else(gen_threads);

    for n in &threads {
        let m = workloads::create(options, *n).run_silently::<C>();
        handler(name, *n, &m);
        gc_cycle(options);
    }
    println!();
}

fn run(options: &Options, h: &mut Handler) {
    //case::<StdRwLockBTreeMapTable<u64>>("std:sync::RwLock<BTreeMap>", options, h);
    //case::<ParkingLotRwLockBTreeMapTable<u64>>("parking_lot::RwLock<BTreeMap>", options, h);
    case::<CHashMapTable<u64>>("CHashMap", options, h);
    case::<CrossbeamSkipMapTable<u64>>("CrossbeamSkipMap", options, h);

    match options.hasher {
        HasherKind::Std => run_hasher_variant::<RandomState>(options, h),
        HasherKind::AHash => run_hasher_variant::<ahash::RandomState>(options, h),
    }
}

fn run_hasher_variant<H>(options: &Options, h: &mut Handler)
where
    H: Default + Clone + Send + Sync + BuildHasher + 'static,
{
    //case::<StdRwLockStdHashMapTable<u64, H>>("std::sync::RwLock<StdHashMap>", options, h);
    //case::<ParkingLotRwLockStdHashMapTable<u64, H>>("parking_lot::RwLock<StdHashMap>", options, h);
    case::<DashMapTable<u64, H>>("DashMap", options, h);
    case::<FlurryTable<u64, H>>("Flurry", options, h);
    case::<EvmapTable<u64, H>>("Evmap", options, h);
    case::<ContrieTable<u64, H>>("Contrie", options, h);
    case::<SccMapTable<u64, H>>("SccMap", options, h);
}

pub fn bench(options: &Options) {
    // Process custom workload options, if parameters are provided
    if let Err(e) = validate_workload(options) {
        eprintln!("Error validating custom workload: {}", e);
        std::process::exit(1);
    }

    println!("== {:?}", options.workload);

    let mut handler = if options.csv {
        let mut wr = csv::WriterBuilder::new()
            .has_headers(!options.csv_no_headers)
            .from_writer(io::stderr());

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

    run(options, &mut handler);
}
