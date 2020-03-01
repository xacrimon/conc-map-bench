# conc-map-bench

conc-map-bench uses the bustle benchmarking harness. This is a port of the well regarded libcuckoo benchmark.

Implementations benchmarked
- Mutex Std HashMap
- RwLock Std HashMap
- CHashMap
- Contrie
- Flurry
- DashMap v3

Benchmark workloads
- Read heavy
    - 94% Read
    - 2% Insert
    - 3% Update
    - 1% Remove
    - 0% Upsert

- Insert heavy
    - 10% Read
    - 80% Insert
    - 10% Update
    - 0% Remove
    - 0% Upsert

- Update heavy
    - 35% Read
    - 5% Insert
    - 50% Update
    - 5% Remove
    - 5% Upsert
