pub use self::{
    btreemap::ParkingLotRwLockBTreeMapTable, btreemap::StdRwLockBTreeMapTable,
    chashmap::CHashMapTable, contrie::ContrieTable, crossbeam_skiplist::CrossbeamSkipMapTable,
    dashmap::DashMapTable, evmap::EvmapTable, flurry::FlurryTable, scc::SccMapTable,
    std::ParkingLotRwLockStdHashMapTable, std::StdRwLockStdHashMapTable,
    papaya::PapayaTable
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
mod evmap;
mod flurry;
mod scc;
mod std;
mod papaya;

type Value = u32;
