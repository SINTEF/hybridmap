# HybridMap

HybridMap is a Rust™ hybrid map implementation that uses a vector on the memory stack for small maps and a hash map overwise.

As with most hybrid technologies, including two components instead of one is one too many. However, the hybrid solution can provide some value for specific use cases.

HybridMap can be slightly faster for tiny maps, especially short-lived ones living on the memory stack, usually up to 16 entries and without too many lookups.

## Example

HybridMap can be used like most other maps.

```rust
use hybridmap::HybridMap;

let mut map = HybridMap::<i32, &str>::new();
map.insert(1, "one");
map.insert(2, "two");

assert_eq!(map.get(&1), Some(&"one"));
assert_eq!(map.len(), 2);
```

## Benchmarks

The benchmark is unlikely to be representative of your use cases. You might see some of the gains shown below if you create many short-lived small maps. You may also get worse performances than a standard hash map.

You could adapt the benchmarks to your use cases. If you don't know whether you should use this hybrid map or a hashmap, you should go with a hashmap. As the numbers show, HybridMap can be much faster for tiny maps, but the gain quickly shrinks as the map grows.

*Results using a MacBook Pro M1 with Rust 1.94.0:*

| Type   | Map            | Size | Median Time (ns) | Performance Gain |
|--------|----------------|------|------------------|------------------|
| i64    | HashMap        | 1    | 129              |                  |
| i64    | **HybridMap**  | 1    | 32               | x4.03            |
| i64    | HashMap        | 4    | 581              |                  |
| i64    | **HybridMap**  | 4    | 188              | x3.09            |
| i64    | HashMap        | 16   | 2 497            |                  |
| i64    | **HybridMap**  | 16   | 820              | x3.05            |
| i64    | HashMap        | 128  | 20 323           |                  |
| i64    | **HybridMap**  | 128  | 19 039           | x1.07            |
| uuid   | HashMap        | 1    | 189              |                  |
| uuid   | **HybridMap**  | 1    | 126              | x1.50            |
| uuid   | HashMap        | 4    | 841              |                  |
| uuid   | **HybridMap**  | 4    | 432              | x1.95            |
| uuid   | HashMap        | 16   | 3 541            |                  |
| uuid   | **HybridMap**  | 16   | 3 458            | x1.02            |
| uuid   | HashMap        | 128  | 27 422           |                  |
| uuid   | **HybridMap**  | 128  | 28 876           | x0.95            |
| string | HashMap        | 1    | 804              |                  |
| string | **HybridMap**  | 1    | 792              | x1.02            |
| string | HashMap        | 4    | 3 561            |                  |
| string | **HybridMap**  | 4    | 3 162            | x1.13            |
| string | HashMap        | 16   | 15 863           |                  |
| string | **HybridMap**  | 16   | 15 754           | x1.01            |
| string | HashMap        | 128  | 115 980          |                  |
| string | **HybridMap**  | 128  | 113 190          | x1.02            |

In this benchmark, the HybridMap switches to a HashMap internally once it has more than `16` entries. This benchmark is not a very robust benchmark. Benchmarking HybridMap correctly is hard and requires more effort than implementing the crate. As the license says, use at your own risk.

*However for tiny maps, that are short-lived, the performance gain could be more interesting:*

| Type                  | Len     | Median Time (ns) | Performance Gain |
|-----------------------|---------|------------------|------------------|
| HashMap<Uuid,i64>     | 1       | 190              |                  |
| HashMap<Uuid,i64>     | 2       | 405              |                  |
| HybridMap<Uuid,i64,1> | 1       | 68               | x2.79            |
| HybridMap<Uuid,i64,1> | 2       | 183              | x2.21            |
| HybridMap<Uuid,i64,4> | 1       | 63               | x3.02            |
| HybridMap<Uuid,i64,4> | 2       | 99               | x4.09            |


```bash
# Run the benchmarks
cargo bench --bench=hybridmap_bench -- --quick --quiet

# Run this command instead if you have more patience
cargo bench --bench=hybridmap_bench

# Open the results in a browser
open target/criterion/report/index.html
# or
xdg-open target/criterion/report/index.html
```

## Memory Usage

HybridMap has a small memory overhead, the enum variant between the vector and the hashmap and a vector pre-allocated on the stack.

The default vector size on the stack is `8` entries. You may save a tiny bit of memory by adapting the vector size to the number of entries you expect to store in the maps. But a large vector will very quickly be a waste of resources. Consider staying below `20`.

For maps containing very few entries, HybridMap can use a bit less memory than a standard HashMap. Otherwise, the memory usage is similar to a normal hashmap.

*Results from one local run of `benches/hybridmap_memory.rs` (`HybridMap<Uuid, i64, 4>` with 2 entries):*

| Structure | Reported memory delta | Stack size |
|-----------|-----------------------|------------|
| HashMap   | 12 672                | 48 bytes   |
| HybridMap | 8 544                 | 112 bytes  |

You can adapt the `benches/hybridmap_memory.rs` file to your use case.

```bash
# Run the memory benchmark
# You will probably have to run it many times without things in the background
# to get a coherent result.
cargo bench --bench=hybridmap_memory
```

## Why ?

I started benchmarking tiny maps to check whether I should switch from HashMap to BTreeMap for my use case. I also had a naive Vec implementation that was faster despite for small maps. Thus, I made this crate for fun.

The energy savings this crate may bring probably do not compensate for the energy I used to boil water for my tea while implementing this crate. But it was fun.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

 * Inspired by [robjtede/tinymap](https://github.com/robjtede/tinymap/).
 * Use [smallvec](https://github.com/servo/rust-smallvec).