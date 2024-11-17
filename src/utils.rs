//! This module contains utility functions that are used across the project.
//!
//! It includes functions for getting the optimal number of threads based on system capabilities.
//!
//! This module contains the BlueHash-based hash function and optimization logic.
//!
//! The `hash_to_commitment` function applies multiple rounds of hashing to increase randomness and
//! strength of the commitment.

use num_cpus;
use std::cmp;
use BlueHash::DigestSize::{Bit256};
use rand::Rng;
use crate::fft::{LOCAL_N, LOCAL_Q};

// Get the optimal number of threads to use for parallel computations.
pub fn get_optimal_thread_count() -> usize {
    let cpu_cores = num_cpus::get();
    if LOCAL_N > 1000 {
        cmp::min(cpu_cores * 2, 16)
    } else {
        cmp::min(cpu_cores, 8)
    }
}


// Hash input data and apply multiple rounds for better randomness.
pub fn hash_to_commitment(input: &[u8]) -> Vec<u8> {
    let mut hasher = BlueHash::BlueHash::new(Bit256);
    hasher.update(input);
    let mut result = hasher.finalize().to_vec();

    for _ in 0..3 {
        hasher.update(&result);
        result = hasher.finalize().to_vec();
    }
    result
}

fn generate_matrix() -> Vec<Vec<u64>> {
    let mut rng = rand::thread_rng();
    (0..LOCAL_N)
        .map(|_| (0..LOCAL_N).map(|_| rng.gen_range(0..LOCAL_Q)).collect())
        .collect()
}

pub fn generate_params() -> (Vec<Vec<u64>>, Vec<Vec<u64>>) {
    (generate_matrix(), generate_matrix())
}
