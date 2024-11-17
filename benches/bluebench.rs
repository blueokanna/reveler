use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use reveler::{commit, fft, utils};

// 基准测试函数
fn commit_benchmark(c: &mut Criterion) {
    // 生成参数
    let (a_1, b_1) = utils::generate_params();

    let mut rng = rand::thread_rng();
    let m: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();
    let r: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();

    c.bench_function("commitment generation", |b| {
        b.iter(|| {
            commit(
                black_box(&a_1),
                black_box(&b_1),
                black_box(&m),
                black_box(&r),
            )
        })
    });
}

// 定义基准测试组
criterion_group! {
    name = benches;
    config = Criterion::default()
    .measurement_time(Duration::from_secs(15))
    .sample_size(1000);

    targets = commit_benchmark
}
criterion_main!(benches);
