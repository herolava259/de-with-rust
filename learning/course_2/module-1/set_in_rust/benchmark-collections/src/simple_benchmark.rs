use std::{
    collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque},
    time::{Duration, Instant},
};
 
use rand::seq::{IndexedRandom, SliceRandom};

pub fn run_benchmarks<TElement>(num_of_loop: usize, source: &mut Vec<TElement>, f: fn(element: &TElement)) -> Duration 
{

    source.shuffle(&mut rand::rng());
    let length = source.len();

    let start = Instant::now();

    for i in 0..num_of_loop {   
        f(&source[(i as usize) % length]);
    }

    start.elapsed()
}

fn benchmark_push<C, F>(num_of_loop: usize, samples: &mut Vec<i32>, mut collection: C, mut insert: F) -> (Duration, C)
where
    F: FnMut(&mut C, i32),
{
    samples.shuffle(&mut rand::rng());
    let len = samples.len();
    let start = Instant::now();
    for i in 0..num_of_loop {
        insert(&mut collection, samples[i % len]);
    }
    (start.elapsed(), collection)
}


fn benchmark_contains<C, F>(num_experiments: usize, collection: &C, collection_items: &[i32], contains: F) -> Duration
where
    F: Fn(&C, &i32) -> bool,
{
    let mut rng = rand::rng();
    let experiments: Vec<i32> = collection_items
        .sample(&mut rng, num_experiments)
        .copied()
        .collect();
 
    let start = Instant::now();
    for val in &experiments {
        let _ = contains(collection, val);
    }
    start.elapsed()
}



pub fn benchmark_vec_push(num_of_loop: usize, samples: &mut Vec<i32>) -> (Duration, Vec<i32>)
{
    benchmark_push(num_of_loop, samples, Vec::new(), |c, v| c.push(v))
}

pub fn benchmark_vec_contains(num_experiments: usize, collection: &mut Vec<i32>) -> Duration
{
    let items: Vec<i32> = collection.clone();
    benchmark_contains(num_experiments, collection, &items, |c, v| c.contains(v))}

pub fn benchmark_linked_list_push_back(num_of_loop: usize, samples: &mut Vec<i32>) -> (Duration, LinkedList<i32>)
{
    benchmark_push(num_of_loop, samples, LinkedList::new(), |c, v| c.push_back(v))
}

pub fn benchmark_linked_list_contains(num_experiments: usize, collection: &LinkedList<i32>) -> Duration
{
    
    let items: Vec<i32> = collection.iter().copied().collect();
    benchmark_contains(num_experiments, collection, &items, |c, v| c.contains(v))
}


pub fn benchmark_hash_set_insert(num_of_loop: usize, samples: &mut Vec<i32>) -> (Duration, HashSet<i32>)
{
    benchmark_push(num_of_loop, samples, HashSet::new(), |c, v| { c.insert(v); })
}


pub fn benchmark_hash_set_contains(num_experiments: usize, collection: &HashSet<i32>) -> Duration
{
    
    let items: Vec<i32> = collection.iter().copied().collect();
    benchmark_contains(num_experiments, collection, &items, |c, v| c.contains(v))
}

pub fn benchmark_btree_set_insert(num_of_loop: usize, samples: &mut Vec<i32>) -> (Duration, BTreeSet<i32>)
{
    benchmark_push(num_of_loop, samples, BTreeSet::new(), |c, v| { c.insert(v); })
}

pub fn benchmark_btree_set_contains(num_experiments: usize, collection: &BTreeSet<i32>) -> Duration {
    let items: Vec<i32> = collection.iter().copied().collect();
    benchmark_contains(num_experiments, collection, &items, |c, v| c.contains(v))
}

pub fn benchmark_binary_heap_push(num_of_loop: usize, samples: &mut Vec<i32>) -> (Duration, BinaryHeap<i32>) {
    benchmark_push(num_of_loop, samples, BinaryHeap::new(), |c, v| c.push(v))
}

pub fn benchmark_vec_deque_push_back(num_of_loop: usize, samples: &mut Vec<i32>) -> (Duration, VecDeque<i32>) {
    benchmark_push(num_of_loop, samples, VecDeque::new(), |c, v| c.push_back(v))
}


pub struct BenchmarkResult {
    pub data_structure: &'static str,
    pub action: &'static str,
    pub iterations: usize,
    pub duration: Duration,
}
 
impl BenchmarkResult {
    pub fn new(data_structure: &'static str, action: &'static str, iterations: usize, duration: Duration) -> Self {
        Self { data_structure, action, iterations, duration }
    }
}



pub fn benchmark_stats_table<TElement>(results: &[BenchmarkResult]) -> String
{
    let mut output = String::from("|Data Structure | Action | Benchmark | Duration (ms) |\n| --- | --- |\n");

    output.push_str("|---|---|---|---|\n");
    for r in results {
        output.push_str(&format!(
            "| {} | {} | {} | {:.3} |\n",
            r.data_structure,
            r.action,
            r.iterations,
            r.duration.as_secs_f64() * 1000.0,
        ));
    }
    output


}

