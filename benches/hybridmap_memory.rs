use std::mem;

use hybridmap::HybridMap;
use uuid::Uuid;

fn fast_random_uuid(mut rng: impl rand::RngCore) -> Uuid {
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    Uuid::from_bytes(bytes)
}

fn take_sample<const N: usize>(nb_samples: usize) -> (u64, u64) {
    // Get initial memory usage
    let mem_before = sys_info::mem_info().unwrap().free;

    let rng = rand::thread_rng();

    // Create a vector to hold the instances of HybridMap
    let mut vec = Vec::with_capacity(100000);
    for _ in 0..100000 {
        let mut map = HybridMap::<Uuid, i64, N>::new();
        for i in 0..nb_samples {
            map.insert(fast_random_uuid(rng.clone()), i as i64);
        }
        vec.push(map);
    }

    // Get memory usage after creation
    let mem_after_hybridmap = sys_info::mem_info().unwrap().free;
    // Calculate the difference
    let mem_used_hybridmap = mem_before - mem_after_hybridmap;

    // Create a vector to hold the instances of HashMap
    let mut vec = Vec::with_capacity(100000);
    for _ in 0..100000 {
        let mut map = std::collections::HashMap::<Uuid, i64>::new();
        for i in 0..nb_samples {
            map.insert(fast_random_uuid(rng.clone()), i as i64);
        }
        vec.push(map);
    }

    let mem_after_hashmap = sys_info::mem_info().unwrap().free;
    let mem_used_hashmap = mem_after_hybridmap - mem_after_hashmap;

    (mem_used_hybridmap, mem_used_hashmap)
}

fn main() {
    // Size of the Vec in HybridMap
    // 8 is the default value
    // From 7, it uses more memory than HashMap
    const N: usize = 4;

    let (hybridmap, hashmap) = take_sample::<N>(2);
    println!("HybridMap median memory usage: {}", hybridmap);
    println!("HashMap median memory usage: {}", hashmap);

    let size_hybridmap = mem::size_of::<HybridMap<Uuid, i64, N>>();
    let size_hashmap = mem::size_of::<std::collections::HashMap<Uuid, i64>>();
    println!("HybridMap size on the stack: {}", size_hybridmap);
    println!("HashMap size on the stack: {}", size_hashmap);
}
