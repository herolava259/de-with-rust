use core::num;
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;

pub fn run_benchmarks<TElement>(num_of_loop: usize, source: &mut Vec<TElement>,f: fn(destination: &mut Vec<TElement>, element: &TElement) -> TElement, element: TElement) -> Duration {

    let mut rand = rand::rng();

    source.shuffle(&mut rand);
    let length = source.len();

    let mut destination: Vec<TElement> = Vec::new();

    let start = Instant::now();

    for i in 0..num_of_loop {   
        f(&mut destination, &source[(i as usize) % length]);
    }

    let duration = start.elapsed();

    return duration;
}

pub fn benchmark_stats_table<TElement>(num_of_loop: usize, samples: &mut Vec<TElement>)
{
    let mut output = String::from("|Data Structure | Action | Benchmark | Duration (ms) |\n| --- | --- |\n");

    let vec_insert_duration = run_benchmarks(num_of_loop, samples.)

}

