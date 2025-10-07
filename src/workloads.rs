use std::{fmt::Debug, str::FromStr};

use bustle::*;

use super::bench::Options;

#[derive(Debug)]
pub enum WorkloadKind {
    ReadHeavy,
    Exchange,
    RapidGrow,
    Custom,
}

impl FromStr for WorkloadKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ReadHeavy" => Ok(Self::ReadHeavy),
            "Exchange" => Ok(Self::Exchange),
            "RapidGrow" => Ok(Self::RapidGrow),
            "Custom" => Ok(Self::Custom),
            _ => Err("unknown workload"),
        }
    }
}

fn read_heavy(threads: u32) -> Workload {
    let mix = Mix {
        read: 98,
        insert: 1,
        remove: 1,
        update: 0,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.75)
}

fn rapid_grow(threads: u32) -> Workload {
    let mix = Mix {
        read: 5,
        insert: 80,
        remove: 5,
        update: 10,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.0)
}

fn exchange(threads: u32) -> Workload {
    let mix = Mix {
        read: 10,
        insert: 40,
        remove: 40,
        update: 10,
        upsert: 0,
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(25)
        .prefill_fraction(0.75)
}

fn custom(options: &Options, threads: u32) -> Workload {
    // already checked the percentages sum to 100
    let mix = Mix {
        read: options.read.unwrap_or(0),
        insert: options.insert.unwrap_or(0),
        remove: options.remove.unwrap_or(0),
        update: options.update.unwrap_or(0),
        upsert: options.upsert.unwrap_or(0),
    };

    *Workload::new(threads as usize, mix)
        .initial_capacity_log2(options.capacity_log2)
        .prefill_fraction(options.prefill)
}

pub(crate) fn create(options: &Options, threads: u32) -> Workload {
    let mut workload = match options.workload {
        WorkloadKind::ReadHeavy => read_heavy(threads),
        WorkloadKind::Exchange => exchange(threads),
        WorkloadKind::RapidGrow => rapid_grow(threads),
        WorkloadKind::Custom => custom(options, threads),
    };

    workload.operations(options.operations);
    workload
}
