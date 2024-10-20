pub use self::{
    btreemap::ParkingLotRwLockBTreeMapTable, btreemap::StdRwLockBTreeMapTable,
    chashmap::CHashMapTable, contrie::ContrieTable, crossbeam_skiplist::CrossbeamSkipMapTable,
    dashmap::DashMapTable, evmap::EvmapTable, flurry::FlurryTable, scc::SccMapTable,
    std::ParkingLotRwLockStdHashMapTable, std::StdRwLockStdHashMapTable,
    dashmap_next::DashMapTable as DashMapTableNext,
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
mod dashmap_next;
mod evmap;
mod flurry;
mod scc;
mod std;

type Value = u32;
