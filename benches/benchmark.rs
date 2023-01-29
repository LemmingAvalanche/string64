use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use string64::*;

// This should be faster than the next function which just uses string
fn string64_in_hashmap() -> usize {
    let mut sum = 0;
    let mut hm: HashMap<String64, u64>= HashMap::new();
    for i in 0..100_000 {
        let s = i.to_string();
        hm.insert(String64::new(&s).unwrap(), i);
    }
    for i in 0..100_000 {
        let s = i.to_string();
        let value = hm.get(&String64::new(&s).unwrap()).unwrap();
        if value % 15 == 0 {
            sum += 1;
        }
    }
    sum
}


fn string_in_hashmap() -> usize {
    let mut sum = 0;
    let mut hm: HashMap<String, u64>= HashMap::new();
    for i in 0..100_000 {
        let s = i.to_string();
        hm.insert(s, i);
    }
    for i in 0..100_000 {
        let s = i.to_string();
        let value = hm.get(&s).unwrap();
        if value % 15 == 0 {
            sum += 1;
        }
    }
    sum
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

fn criterion_benchmark_string64_in_hashmap(c: &mut Criterion) {
    c.bench_function("string64_in_hashmap", |b| b.iter(|| string64_in_hashmap()));
}

fn criterion_benchmark_string_in_hashmap(c: &mut Criterion) {
    c.bench_function("string_in_hashmap", |b| b.iter(|| string_in_hashmap()));
}

criterion_group!(benches, criterion_benchmark_string64_in_hashmap, criterion_benchmark_string_in_hashmap);
criterion_main!(benches);
