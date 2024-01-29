# HybridMap

HybridMap is a Rust™ hybrid map implementation that uses a vector for small maps and a hash map overwise.

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

You could adapt the benchmarks to your use cases. If you know whether you should use this hybrid map or a hashmap, you should go with a hashmap. As the numbers show, the performance gain is not that great.

*Results on a Macbook Pro M1:*

| Type   | Map            | Size | Median Time (ns) | Performance Gain |
| ------ | -------------- | ---- | ---------------- | ---------------- |
| i64    | HashMap        | 1    | 244              |                  |
| i64    | **HybridMap**  | 1    | 188              | x1.29            |
| i64    | HashMap        | 4    | 1 107            |                  |
| i64    | **HybridMap**  | 4    | 800              | x1.38            |
| i64    | HashMap        | 16   | 4 543            |                  |
| i64    | **HybridMap**  | 16   | 3 233            | x1.41            |
| i64    | HashMap        | 128  | 36 633           |                  |
| i64    | **HybridMap**  | 128  | 36 695           | x1.0             |
| uuid   | HashMap        | 1    | 347              |                  |
| uuid   | **HybridMap**  | 1    | 235              | x1.48            |
| uuid   | HashMap        | 4    | 1 604            |                  |
| uuid   | **HybridMap**  | 4    | 936              | x1.71            |
| uuid   | HashMap        | 16   | 6 331            |                  |
| uuid   | **HybridMap**  | 16   | 6 448            | x0.98            |
| uuid   | HashMap        | 128  | 47 510           |                  |
| uuid   | **HybridMap**  | 128  | 49 862           | x0.95            |
| string | HashMap        | 1    | 1 189            |                  |
| string | **HybridMap**  | 1    | 1 108            | x1.07            |
| string | HashMap        | 4    | 5 292            |                  |
| string | **HybridMap**  | 4    | 4 496            | x1.18            |
| string | HashMap        | 16   | 20 586           |                  |
| string | **HybridMap**  | 16   | 20 695           | x0.99            |
| string | HashMap        | 128  | 156 250          |                  |
| string | **HybridMap**  | 128  | 156 600          | x1.0             |

After `16` entries, HybridMap switches to a HashMap internally. This benchmark is not a very robust benchmark. Benchmarking HybridMap correctly is hard and requires more effort than implementing the crate. As the license says, use at your own risk.

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

The default vector size on the stack is 8 entries. You may save a tiny bit of memory by adapting the vector size to the number of entries you expect to store in the maps. But a large vector will very quickly be a waste of resources. Consider staying below 20.

For maps containing very few entries, one or two, memory usage can be one order of magnitude smaller than a hashmap. Otherwise, the memory usage is similar to a normal hashmap.

You can adapt the `benches/hybridmap_memory.rs` file to your use case.

```bash
# Run the memory benchmark
# You will probably have to run it many times without things in the background
# to get a coherent result.
cargo bench --bench=hybridmap_memory
```

## Why ?

I started benchmarking tiny maps to check whether I should switch from HashMap to BTreeMap for my use case. I also had a naive Vec implementation that was surprisingly faster. Thus, I made this crate for fun.

The energy savings this crate may bring probably do not compensate for the energy I used to boil water for my tea while implementing this crate. But it was fun.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

 * Inspired by [robjtede/tinymap/](https://github.com/robjtede/tinymap/).
 * Use [smallvec](https://github.com/servo/rust-smallvec).