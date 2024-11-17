use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use reveler::{fft, utils, RevelerCommit};

/// 基准测试 - 承诺生成
fn commit_benchmark(c: &mut Criterion) {
    // 生成随机参数
    let (a_1, b_1) = utils::generate_params();
    let mut rng = rand::thread_rng();
    let m: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();
    let r: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();

    c.bench_function("commitment generation", |b| {
        b.iter(|| {
            let commit = RevelerCommit::new(
                black_box(a_1.clone()),
                black_box(b_1.clone()),
                black_box(m.clone()),
                black_box(r.clone()),
            );
            commit.commit().unwrap();
        });
    });
}

/// 基准测试 - 承诺验证
fn verify_benchmark(c: &mut Criterion) {
    // 生成随机参数并生成承诺
    let (a_1, b_1) = utils::generate_params();
    let mut rng = rand::thread_rng();
    let m: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();
    let r: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();

    let commit = RevelerCommit::new(a_1, b_1, m, r);
    let result = commit.commit().unwrap();

    c.bench_function("commitment verification", |b| {
        b.iter(|| {
            RevelerCommit::verify(black_box(&result));
        });
    });
}


criterion_group! {
    name = benches;
    config = Criterion::default()
        .measurement_time(Duration::from_secs(10))
        .sample_size(500);
    targets = commit_benchmark, verify_benchmark
}
criterion_main!(benches);
