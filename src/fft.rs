//! This module provides the Fast Fourier Transform (FFT) operations required for matrix multiplication.
//!
//! It uses the `rustfft` crate to perform forward and inverse FFT transformations and applies them
//! to the matrix multiplication used in commitment generation.

use rustfft::{FftPlanner, num_complex::Complex};

pub const LOCAL_N: usize = 256;
pub const LOCAL_Q: u64 = u16::MAX as u64;

// Perform FFT matrix multiplication.
pub fn fft_matrix_multiply(row: &[u64], v: &[u64]) -> Vec<u64> {
    let mut planner = FftPlanner::<f64>::new();
    let fft = planner.plan_fft_forward(LOCAL_N);

    let mut row_fft: Vec<Complex<f64>> = row.iter().map(|&x| Complex::new(x as f64, 0.0)).collect();
    let mut v_fft: Vec<Complex<f64>> = v.iter().map(|&x| Complex::new(x as f64, 0.0)).collect();

    fft.process(&mut row_fft);
    fft.process(&mut v_fft);

    let mut result_fft: Vec<Complex<f64>> = row_fft
        .iter()
        .zip(&v_fft)
        .map(|(a, b)| a * b)
        .collect();

    let ifft = planner.plan_fft_inverse(LOCAL_N);
    ifft.process(&mut result_fft);

    let scale_factor = 1.0 / (LOCAL_N as f64);
    result_fft
        .iter()
        .map(|x| {
            let scaled_value = (x.re * scale_factor).round() as i64;
            ((scaled_value % LOCAL_Q as i64 + LOCAL_Q as i64) % LOCAL_Q as i64) as u64 // 确保结果为非负数
        })
        .collect()
}