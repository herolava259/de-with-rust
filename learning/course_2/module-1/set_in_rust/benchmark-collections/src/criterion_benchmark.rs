use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};

use rand::seq::SliceRandom;

use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};


const SIZES: &[usize] = &[100, 1_000, 10_000, 100_000];


fn make_samples(n: usize) -> Vec<i32> {

    let mut v: Vec<i32> = (0..n as i32).collect();

    v.shuffle(&mut rand::rng());

    v
}

fn filled_vec(samples: &[i32]) -> Vec<i32> {

    samples.to_vec()
}


fn filled_linked_list(samples: &[i32]) -> LinkedList<i32> {
    samples.iter().copied().collect()
}


fn filled_hash_set(samples: &[i32]) -> HashSet<i32> {
    samples.iter().copied().collect()
}

fn filled_btree_set(samples: &[i32]) -> BTreeSet<i32> {
    samples.iter().copied().collect()
}

fn filled_binary_heap(samples: &[i32]) -> BinaryHeap<i32> {
    samples.iter().copied().collect()
}


fn filled_vec_deque(samples: &[i32]) -> VecDeque<i32> {
    samples.iter().copied().collect()
}

pub fn bench_push(c: &mut Criterion) {

    let mut group = c.benchmark_group("push_insert");

    for &size in SIZES {
        let samples = make_samples(size);

        group.bench_with_input(BenchmarkId::new("Vec", size), &samples, |b, s| {

            b.iter_batched(|| s.clone(), 
                            |data| {
                                let mut col = Vec::with_capacity(data.len());
                                for v in data {
                                    col.push(v);
                                }
                                col
                            },
                            BatchSize::SmallInput);      
        });

        group.bench_with_input(BenchmarkId::new("LinkedList", size), &samples, |b, s|{
            b.iter_batched(|| s.clone(), 
                            |data| {
                                let mut col = LinkedList::new();
                                for v in data {
                                    col.push_back(v);
                                }
                                col
                            }, BatchSize::SmallInput
                        );        
        });

        group.bench_with_input(BenchmarkId::new("HashSet", size), &samples, |b, s|{
            b.iter_batched(|| s.clone(), 
                            |data| {
                                let mut col = HashSet::with_capacity(data.len());
                                for v in data {
                                    col.insert(v);
                                }
                                col
                            }, BatchSize::SmallInput
                        );        
        });

        group.bench_with_input(BenchmarkId::new("BTreeSet", size), &samples, |b, s|{
            b.iter_batched(|| s.clone(), 
                            |data| {
                                let mut col = BTreeSet::new();
                                for v in data {
                                    col.insert(v);
                                }
                                col
                            }, BatchSize::SmallInput
                        );        
        });


        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), &samples, |b, s|{
            b.iter_batched(|| s.clone(), 
                            |data| {
                                let mut col = BinaryHeap::with_capacity(data.len());
                                for v in data {
                                    col.push(v);
                                }
                                col
                            }, BatchSize::SmallInput
                        );        
        });

        group.bench_with_input(BenchmarkId::new("VecDeque", size), &samples, |b, s|{
            b.iter_batched(|| s.clone(), 
                            |data| {
                                let mut col = VecDeque::with_capacity(data.len());
                                for v in data {
                                    col.push_back(v);
                                }
                                col
                            }, BatchSize::SmallInput
                        );        
        });


    }

    group.finish();

}


pub fn bench_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains");

    for &size in SIZES {
        let samples = make_samples(size);

        let needle = samples[size / 2];

        group.bench_with_input(BenchmarkId::new("Vec", size), &samples, |b, s| {
            let col = filled_vec(s);

            b.iter(|| col.contains(&needle))
        });

        group.bench_with_input(BenchmarkId::new("LinkedList", size), &samples, |b, s| {
            let col = filled_linked_list(s);
            b.iter(|| col.contains(&needle))
        });
 
        group.bench_with_input(BenchmarkId::new("HashSet", size), &samples, |b, s| {
            let col = filled_hash_set(s);
            b.iter(|| col.contains(&needle))
        });
 
        group.bench_with_input(BenchmarkId::new("BTreeSet", size), &samples, |b, s| {
            let col = filled_btree_set(s);
            b.iter(|| col.contains(&needle))
        });

        group.bench_with_input(BenchmarkId::new("BinaryHeap", size), &samples, |b, s| {
            let col = filled_binary_heap(s);
            b.iter(|| col.iter().any(|x| *x == needle))
        });
 
        group.bench_with_input(BenchmarkId::new("VecDeque", size), &samples, |b, s| {
            let col = filled_vec_deque(s);
            b.iter(|| col.contains(&needle))
        });
    }

    group.finish();
}

pub fn bench_pop(c: &mut Criterion) {
    let mut group = c.benchmark_group("pop_remove");
 
    for &size in SIZES {
        let samples = make_samples(size);
 
        // Vec::pop  (removes from tail — O(1))
        group.bench_with_input(BenchmarkId::new("Vec_pop", size), &samples, |b, s| {
            b.iter_batched(
                || filled_vec(s),
                |mut col| {
                    while col.pop().is_some() {}
                },
                BatchSize::SmallInput,
            )
        });
 
        // Vec::remove(0)  (removes from head — O(n))
        group.bench_with_input(BenchmarkId::new("Vec_remove_front", size), &samples, |b, s| {
            b.iter_batched(
                || filled_vec(s),
                |mut col| {
                    while !col.is_empty() {
                        col.remove(0);
                    }
                },
                BatchSize::SmallInput,
            )
        });
 
        // LinkedList::pop_back
        group.bench_with_input(BenchmarkId::new("LinkedList_pop_back", size), &samples, |b, s| {
            b.iter_batched(
                || filled_linked_list(s),
                |mut col| {
                    while col.pop_back().is_some() {}
                },
                BatchSize::SmallInput,
            )
        });
 
        // LinkedList::pop_front
        group.bench_with_input(BenchmarkId::new("LinkedList_pop_front", size), &samples, |b, s| {
            b.iter_batched(
                || filled_linked_list(s),
                |mut col| {
                    while col.pop_front().is_some() {}
                },
                BatchSize::SmallInput,
            )
        });
 
        // HashSet::remove (by value)
        group.bench_with_input(BenchmarkId::new("HashSet_remove", size), &samples, |b, s| {
            b.iter_batched(
                || (filled_hash_set(s), s.clone()),
                |(mut col, keys)| {
                    for k in &keys {
                        col.remove(k);
                    }
                },
                BatchSize::SmallInput,
            )
        });
 
        // BTreeSet::remove (by value)
        group.bench_with_input(BenchmarkId::new("BTreeSet_remove", size), &samples, |b, s| {
            b.iter_batched(
                || (filled_btree_set(s), s.clone()),
                |(mut col, keys)| {
                    for k in &keys {
                        col.remove(k);
                    }
                },
                BatchSize::SmallInput,
            )
        });
 
        // BinaryHeap::pop (always removes the max)
        group.bench_with_input(BenchmarkId::new("BinaryHeap_pop", size), &samples, |b, s| {
            b.iter_batched(
                || filled_binary_heap(s),
                |mut col| {
                    while col.pop().is_some() {}
                },
                BatchSize::SmallInput,
            )
        });
 
        // VecDeque::pop_back
        group.bench_with_input(BenchmarkId::new("VecDeque_pop_back", size), &samples, |b, s| {
            b.iter_batched(
                || filled_vec_deque(s),
                |mut col| {
                    while col.pop_back().is_some() {}
                },
                BatchSize::SmallInput,
            )
        });
 
        // VecDeque::pop_front
        group.bench_with_input(BenchmarkId::new("VecDeque_pop_front", size), &samples, |b, s| {
            b.iter_batched(
                || filled_vec_deque(s),
                |mut col| {
                    while col.pop_front().is_some() {}
                },
                BatchSize::SmallInput,
            )
        });
    }
 
    group.finish();
}


criterion_group!(benches, bench_push, bench_contains, bench_pop);
criterion_main!(benches);