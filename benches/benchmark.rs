pub fn create_100_vec() -> Vec<usize> {
    let mut res = Vec::with_capacity(100);

    for i in 1..101 {
        res.push(i);
    }

    res
}

pub fn print_without_zeros(primes: Vec<usize>) {
    for i in 0..100 {
        let num = primes[i];

        if num != 0 {
            println!("{}", num);
        }
    }
}

pub fn find_primes_exclusion() -> Vec<usize> {
    let mut numbers = create_100_vec();

    for i in 2..101 {
        if numbers[i - 1] == 0 {
            continue;
        }

        let mut j = 2;

        while i * j <= 100 {
            numbers[i * j - 1] = 0;
            j += 1;
        }
    }

    numbers
}

pub fn find_primes_check() -> Vec<usize> {
    let mut numbers = create_100_vec();

    for i in 2..101 {
        if numbers[i - 1] == 0 {
            continue;
        }

        for j in 2..i {
            if numbers[i] % j == 0 {
                numbers[i - 1] = 0;
            }
        }
    }

    numbers
}

use criterion::{criterion_group, criterion_main, Criterion};

fn create_100_vec_benchmark(c: &mut Criterion) {
    c.bench_function("create vector of numbers from 1 to 100", |b| {
        b.iter(|| create_100_vec())
    });
}

fn print_without_zeros_benchmark(c: &mut Criterion) {
    c.bench_function("print numbers that are not 0 in vector", |b| {
        b.iter(|| print_without_zeros(criterion::black_box(vec![0; 100])))
    });
}

fn find_primes_exclusion_benchmark(c: &mut Criterion) {
    c.bench_function("find prime numbers using exclusion", |b| {
        b.iter(|| find_primes_exclusion())
    });
}

fn find_primes_check_benchmark(c: &mut Criterion) {
    c.bench_function("find prime numbers using checks", |b| {
        b.iter(|| find_primes_exclusion())
    });
}

criterion_group!(
    benches,
    create_100_vec_benchmark,
    print_without_zeros_benchmark,
    find_primes_exclusion_benchmark,
    find_primes_check_benchmark,
);

criterion_main!(benches);
