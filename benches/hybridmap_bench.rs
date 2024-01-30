use criterion::{criterion_group, criterion_main, Criterion};
use hybridmap::HybridMap;
use rand::{distributions::DistString, Rng};
use std::collections::HashMap;
use uuid::Uuid;

fn fast_random_uuid(mut rng: impl rand::RngCore) -> Uuid {
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);
    Uuid::from_bytes(bytes)
}

#[inline]
fn random_string(mut rng: impl rand::RngCore, len: usize) -> String {
    rand::distributions::Alphanumeric.sample_string(&mut rng, len)
}

fn hybridmap_bench(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("i64");
    for size in [1, 4, 16, 128].iter() {
        group.bench_function(format!("HybridMap {}", size), |b| {
            b.iter(|| {
                let mut map = HybridMap::<i64, i64, 16>::new();

                let mut sum = 0_i64;
                for _i in 0..criterion::black_box(*size * 2) {
                    map.insert(rng.gen_range(0..*size), rng.gen_range(0..*size));
                    let n = map.get(&rng.gen_range(0..*size));
                    if let Some(n) = n {
                        sum += n;
                    }
                }
            })
        });
        group.bench_function(format!("HashMap {}", size), |b| {
            b.iter(|| {
                let mut map = HashMap::<i64, i64>::new();

                let mut sum = 0_i64;
                for _i in 0..criterion::black_box(*size * 2) {
                    map.insert(rng.gen_range(0..*size), rng.gen_range(0..*size));
                    let n = map.get(&rng.gen_range(0..*size));
                    if let Some(n) = n {
                        sum += n;
                    }
                }
            })
        });
    }
    group.finish();

    let mut group = c.benchmark_group("uuid");
    for size in [1, 4, 16, 128].iter() {
        group.bench_function(format!("HybridMap {}", size), |b| {
            b.iter(|| {
                let mut map = HybridMap::<Uuid, i64, 16>::new();
                let mut uuids_pool: Vec<Uuid> = Vec::new();

                let mut sum = 0_i64;
                for _i in 0..criterion::black_box(*size * 2) {
                    let uuid = fast_random_uuid(&mut rng);
                    map.insert(uuid, rng.gen_range(0..*size));
                    uuids_pool.push(uuid);

                    // 50% chance to get a random uuid from the pool
                    if rng.gen_bool(0.5) {
                        let n = map.get(&uuids_pool[rng.gen_range(0..uuids_pool.len())]);
                        if let Some(n) = n {
                            sum += n;
                        }
                    } else {
                        let n = map.get(&fast_random_uuid(&mut rng));
                        if let Some(n) = n {
                            sum += n;
                        }
                    }
                }
            })
        });
        group.bench_function(format!("HashMap {}", size), |b| {
            b.iter(|| {
                let mut map = HashMap::<Uuid, i64>::new();
                let mut uuids_pool: Vec<Uuid> = Vec::new();

                let mut sum = 0_i64;
                for _i in 0..criterion::black_box(*size * 2) {
                    let uuid = fast_random_uuid(&mut rng);
                    map.insert(uuid, rng.gen_range(0..*size));
                    uuids_pool.push(uuid);

                    // 50% chance to get a random uuid from the pool
                    if rng.gen_bool(0.5) {
                        let n = map.get(&uuids_pool[rng.gen_range(0..uuids_pool.len())]);
                        if let Some(n) = n {
                            sum += n;
                        }
                    } else {
                        let n = map.get(&fast_random_uuid(&mut rng));
                        if let Some(n) = n {
                            sum += n;
                        }
                    }
                }
            })
        });
    }

    group.finish();

    let mut group = c.benchmark_group("string");
    for size in [1, 4, 16, 128].iter() {
        group.bench_function(format!("HybridMap {}", size), |b| {
            b.iter(|| {
                let mut map = HybridMap::<String, i64, 16>::new();
                let mut strings_pool: Vec<String> = Vec::new();

                let mut sum = 0_i64;
                for _i in 0..criterion::black_box(*size * 2) {
                    let string_size = rng.gen_range(4..64);
                    let string = random_string(&mut rng, string_size);
                    map.insert(string.clone(), rng.gen_range(0..*size));
                    strings_pool.push(string);

                    // 50% chance to get a random string from the pool
                    if rng.gen_bool(0.5) {
                        let n = map.get(&strings_pool[rng.gen_range(0..strings_pool.len())]);
                        if let Some(n) = n {
                            sum += n;
                        }
                    } else {
                        let n = map.get(&random_string(&mut rng, 16));
                        if let Some(n) = n {
                            sum += n;
                        }
                    }
                }
            })
        });
        group.bench_function(format!("HashMap {}", size), |b| {
            b.iter(|| {
                let mut map = HashMap::<String, i64>::new();
                let mut strings_pool: Vec<String> = Vec::new();

                let mut sum = 0_i64;
                for _i in 0..criterion::black_box(*size * 2) {
                    let string_size = rng.gen_range(4..64);
                    let string = random_string(&mut rng, string_size);
                    map.insert(string.clone(), rng.gen_range(0..*size));
                    strings_pool.push(string);

                    // 50% chance to get a random string from the pool
                    if rng.gen_bool(0.5) {
                        let n = map.get(&strings_pool[rng.gen_range(0..strings_pool.len())]);
                        if let Some(n) = n {
                            sum += n;
                        }
                    } else {
                        let n = map.get(&random_string(&mut rng, 16));
                        if let Some(n) = n {
                            sum += n;
                        }
                    }
                }
            })
        });
    }

    group.finish();

    let mut group = c.benchmark_group("init");

    group.bench_function("HybridMap<1> 1", |b| {
        b.iter(|| {
            let mut map = HybridMap::<Uuid, i64, 1>::new();
            map.insert(fast_random_uuid(&mut rng), 1);
            let collected_vec: Vec<_> = map.iter().collect();
            assert_eq!(collected_vec.len(), 1);
        })
    });
    group.bench_function("HybridMap<1> 2", |b| {
        b.iter(|| {
            let mut map = HybridMap::<Uuid, i64, 1>::new();
            map.insert(fast_random_uuid(&mut rng), 1);
            map.insert(fast_random_uuid(&mut rng), 2);
            let collected_vec: Vec<_> = map.iter().collect();
            assert_eq!(collected_vec.len(), 2);
        })
    });

    group.bench_function("HybridMap<4> 1", |b| {
        b.iter(|| {
            let mut map = HybridMap::<Uuid, i64, 4>::new();
            map.insert(fast_random_uuid(&mut rng), 1);
            let collected_vec: Vec<_> = map.iter().collect();
            assert_eq!(collected_vec.len(), 1);
        })
    });

    group.bench_function("HybridMap<4> 2", |b| {
        b.iter(|| {
            let mut map = HybridMap::<Uuid, i64, 4>::new();
            map.insert(fast_random_uuid(&mut rng), 1);
            map.insert(fast_random_uuid(&mut rng), 2);
            let collected_vec: Vec<_> = map.iter().collect();
            assert_eq!(collected_vec.len(), 2);
        })
    });

    group.bench_function("HashMap 1", |b| {
        b.iter(|| {
            let mut map = HashMap::<Uuid, i64>::new();
            map.insert(fast_random_uuid(&mut rng), 1);
            let collected_vec: Vec<_> = map.iter().collect();
            assert_eq!(collected_vec.len(), 1);
        })
    });
    group.bench_function("HashMap 2", |b| {
        b.iter(|| {
            let mut map = HashMap::<Uuid, i64>::new();
            map.insert(fast_random_uuid(&mut rng), 1);
            map.insert(fast_random_uuid(&mut rng), 2);
            let collected_vec: Vec<_> = map.iter().collect();
            assert_eq!(collected_vec.len(), 2);
        })
    });

    group.finish();
}

criterion_group!(benches, hybridmap_bench);
criterion_main!(benches);
