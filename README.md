# conc-map-bench

conc-map-bench uses the bustle benchmarking harness. This is a port of the well regarded libcuckoo benchmark.

Implementations benchmarked
- Mutex Std HashMap
- CHashMap
- Contrie
- Flurry
- DashMap

## Models

### Cache

A read heavy model with few inserts, removals and updates. Models caching of data in places such as webservers and disk page caches.

```rust
Self {
    read: 94,
    insert: 2,
    update: 3,
    remove: 1,
    upsert: 0,
}
```

### Rapid grow

An insert heavy model that replicates load in a scenario where the map is used to gather large amounts of data
under a short burst.

```rust
Mix {
    read: 10,
    insert: 40,
    remove: 40,
    update: 10,
    upsert: 0,
}
```

### Exchange

Insert and remove heavy model that replicates a scenario where the map is used to exchange data.

```rust
Mix {
    read: 10,
    insert: 40,
    remove: 40,
    update: 10,
    upsert: 0,
}
```
