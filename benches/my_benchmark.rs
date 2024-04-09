use basic_reverb::{
    fibonacci,
    mix_matrix::hadamard::{self, Hadamard},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("hadamard", |bencher| {
        bencher.iter(|| {
            let mut data = [
                0.31758470, 0.59646898, 0.54462824, 0.03693621, 0.80418398, 0.18855856, 0.51547662,
                0.24771104,
            ];
            Hadamard::in_place(black_box(&mut data));
        })
    });
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
