//! This is a library for cryptographic commitment schemes and verification using optimized FFT and hashing functions.
//!
//! This module provides the functions for commitment generation and verification.
//! It provides the following functions:
//! - `commit`: to generate a cryptographic commitment.
//! - `verify`: to verify a commitment.
//! function enhances the verification process by including a random challenge.
//!
//! You can check repository from github: https://github.com/blueokanna/BlueHash for more details.

pub mod fft;
pub mod utils;

use std::{cmp, thread};
use std::sync::{Arc, Mutex};
use rand::Rng;

/// Struct to hold the commitment point and its corresponding hash.
#[derive(Debug, Clone)]
pub struct RevelerCommit {
    pub commitment_point: Vec<u64>, // The commitment point computed from input parameters
    pub commitment_hash: Vec<u8>,   // The hash of the commitment point
}

impl RevelerCommit {
    /// Constructor to create a new RevelerCommit with a given commitment point and hash.
    ///
    /// # Parameters
    /// - `commitment_point`: A vector of u64 representing the computed commitment point.
    /// - `commitment_hash`: A vector of u8 representing the hash of the commitment point.
    ///
    /// # Returns
    /// A new instance of RevelerCommit.
    pub fn new(commitment_point: Vec<u64>, commitment_hash: Vec<u8>) -> Self {
        RevelerCommit {
            commitment_point,
            commitment_hash,
        }
    }
}

impl Default for RevelerCommit {
    /// Default constructor for RevelerCommit that generates random parameters and computes the commitment.
    ///
    /// This will use the `generate_params` function to generate `a` and `b`, and then compute the
    /// commitment point and hash based on these parameters.
    ///
    /// # Returns
    /// A new instance of RevelerCommit with randomly generated parameters.
    fn default() -> Self {
        let (a, b) = utils::generate_params(); // Generate random parameters for commitment

        let mut rng = rand::thread_rng();
        let m: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect(); // Random m vector
        let r: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect(); // Random r vector

        let commitment_point = commit(&a, &b, &m, &r).commitment_point; // Compute commitment point

        let commitment_hash = utils::hash_to_commitment(
            &commitment_point.iter().flat_map(|&x| x.to_be_bytes()).collect::<Vec<u8>>(), // Hash the commitment point
        );

        RevelerCommit::new(commitment_point, commitment_hash)
    }
}

// Commitment function to create a commitment from input parameters.
pub fn commit(a: &[Vec<u64>], b: &[Vec<u64>], m: &[u64], r: &[u64]) -> RevelerCommit {
    let thread_count = utils::get_optimal_thread_count(); // Determine the optimal thread count for parallelization
    let commitment_point = Arc::new(Mutex::new(vec![0u64; fft::LOCAL_N])); // Initialize the commitment point

    let mut handles = vec![];
    let chunk_size = (fft::LOCAL_N + thread_count - 1) / thread_count; // Divide work into chunks for each thread

    // Spawn threads to compute commitment point in parallel
    for thread_id in 0..thread_count {
        let start = thread_id * chunk_size;
        let end = cmp::min((thread_id + 1) * chunk_size, fft::LOCAL_N);

        if start >= end {
            continue;
        }

        let a_chunk = a[start..end].to_vec();
        let b_chunk = b[start..end].to_vec();
        let m = m.to_vec();
        let r = r.to_vec();
        let commitment_point = Arc::clone(&commitment_point);

        let handle = thread::spawn(move || {
            // Perform matrix multiplication and compute commitment point
            for (i, (a_row, b_row)) in a_chunk.iter().zip(b_chunk.iter()).enumerate() {
                let m_res = fft::fft_matrix_multiply(a_row, &m); // FFT matrix multiplication on a_row
                let r_res = fft::fft_matrix_multiply(b_row, &r); // FFT matrix multiplication on b_row

                let sum: u64 = m_res.iter().zip(r_res.iter())
                    .fold(0u64, |acc, (&x, &y)| (acc + x + y) % fft::LOCAL_Q); // Combine results and compute sum

                let mut commitment = commitment_point.lock().unwrap(); // Lock the commitment point to update
                commitment[start + i] = sum; // Store the computed sum in the commitment point
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish their work
    for handle in handles {
        handle.join().unwrap();
    }

    // Retrieve the final commitment point after all threads have finished
    let commitment_point = Arc::try_unwrap(commitment_point)
        .unwrap()
        .into_inner()
        .unwrap();

    // Compute the commitment hash from the commitment point
    let commitment_hash = utils::hash_to_commitment(
        &commitment_point
            .iter()
            .flat_map(|&x| x.to_be_bytes())
            .collect::<Vec<u8>>(),
    );

    RevelerCommit::new(commitment_point, commitment_hash)
}

/// Function to verify the validity of a given commitment.
///
/// # Parameters
/// - `commitment`: The commitment to verify.
///
/// # Returns
/// - `true` if the commitment is valid, otherwise `false`.
pub fn verify(commitment: &RevelerCommit) -> bool {
    // Recompute the commitment hash based on the commitment point
    let recomputed_commitment_hash = utils::hash_to_commitment(
        &commitment.commitment_point.iter().flat_map(|&x| x.to_be_bytes()).collect::<Vec<u8>>(),
    );

    // Check if the recomputed hash matches the original commitment hash
    recomputed_commitment_hash == commitment.commitment_hash
}
