//! This is a library for cryptographic commitment schemes and verification using optimized FFT and hashing functions.
//!
//! This module provides the functions for commitment generation and verification.
//! It provides the following functions:
//! - `commit`: to generate a cryptographic commitment.
//! - `verify`: to verify a commitment.
//! Function enhances the verification process by including a random challenge.
//!
//! You can check repository from github: https://github.com/blueokanna/BlueHash for more details.

pub mod fft;
pub mod utils;
pub mod commit_error;

use std::{cmp, thread};
use serde::{Serialize, Deserialize};
use rand::Rng;
use crate::commit_error::CommitError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevelerResult {
    pub commitment_point: Vec<u64>, // The commitment point computed from input parameters
    pub commitment_hash: Vec<u8>,   // The hash of the commitment point
}

impl RevelerResult {
    pub fn new(commitment_point: Vec<u64>, commitment_hash: Vec<u8>) -> Self {
        RevelerResult {
            commitment_point,
            commitment_hash,
        }
    }
}

/// Struct to hold the commitment point and its corresponding hash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevelerCommit {
    pub local_a: Vec<Vec<u64>>,
    pub local_b: Vec<Vec<u64>>,
    pub local_m: Vec<u64>,
    pub local_r: Vec<u64>,
}

impl RevelerCommit {
    /// Constructor to create a new `RevelerCommit` with a given commitment point and hash.
    ///
    /// # Parameters
    /// - `commitment_point`: A vector of `u64` representing the computed commitment point.
    /// - `commitment_hash`: A vector of `u8` representing the hash of the commitment point.
    ///
    /// # Returns
    /// A new instance of `RevelerCommit`.
    pub fn new(local_a: Vec<Vec<u64>>, local_b: Vec<Vec<u64>>, local_m: Vec<u64>, local_r: Vec<u64>) -> Self {
        RevelerCommit {
            local_a,
            local_b,
            local_m,
            local_r,
        }
    }

    /// Generates a cryptographic commitment using parallel computation.
    ///
    /// # Parameters
    /// - `local_a`: A reference to a 2D vector of `u64` representing the first matrix parameter.
    /// - `local_b`: A reference to a 2D vector of `u64` representing the second matrix parameter.
    /// - `local_m`: A reference to a vector of `u64` values representing the message vector.
    /// - `local_r`: A reference to a vector of `u64` values representing the randomness vector.
    ///
    /// # Returns
    /// A `RevelerCommit` containing the computed commitment point and its hash.
    pub fn commit(&self) -> Result<RevelerResult, CommitError> {
        let thread_count = utils::get_optimal_thread_count();
        let chunk_size = (fft::LOCAL_N + thread_count - 1) / thread_count;

        let mut thread_results = vec![vec![0u64; chunk_size]; thread_count];

        let handles: Vec<_> = (0..thread_count).map(|thread_id| {
            let start = thread_id * chunk_size;
            let end = cmp::min((thread_id + 1) * chunk_size, fft::LOCAL_N);

            let a_chunk = self.local_a[start..end].to_vec();
            let b_chunk = self.local_b[start..end].to_vec();
            let m = self.local_m.clone();
            let r = self.local_r.clone();

            thread::spawn(move || {
                let mut result_chunk = vec![0u64; end - start];
                for (i, (a_row, b_row)) in a_chunk.iter().zip(b_chunk.iter()).enumerate() {
                    let m_res = fft::fft_matrix_multiply(a_row, &m);
                    let r_res = fft::fft_matrix_multiply(b_row, &r);

                    result_chunk[i] = m_res.iter().zip(r_res.iter())
                        .fold(0u64, |acc, (&x, &y)| (acc + x + y) % fft::LOCAL_Q);
                }
                result_chunk
            })
        }).collect();

        // 合并线程结果
        for (thread_id, handle) in handles.into_iter().enumerate() {
            thread_results[thread_id] = handle.join().unwrap_or_else(|_| vec![]);
        }

        let commitment_point: Vec<u64> = thread_results.into_iter().flatten().collect();

        // 计算哈希
        let commitment_hash = utils::hash_to_commitment(
            &commitment_point.iter().flat_map(|&x| x.to_be_bytes()).collect::<Vec<u8>>(),
        );

        Ok(RevelerResult::new(commitment_point, commitment_hash))
    }

    /// Verifies the validity of a given cryptographic commitment.
    ///
    /// # Parameters
    /// - `commitment`: A reference to a `RevelerCommit` instance to be verified.
    ///
    /// # Returns
    /// `true` if the commitment is valid, otherwise `false`.
    pub fn verify(commitment: &RevelerResult) -> bool {
        let recomputed_commitment_hash = utils::hash_to_commitment(
            &commitment.commitment_point.iter().flat_map(|&x| x.to_be_bytes()).collect::<Vec<u8>>(),
        );

        recomputed_commitment_hash == commitment.commitment_hash
    }
}

/// Creates a default `RevelerCommit` using randomly generated parameters.
///
/// This constructor generates random `a` and `b` matrices, and computes the
/// commitment point and its hash based on these parameters.
///
/// # Returns
/// A default instance of `RevelerCommit`.
impl Default for RevelerResult {
    fn default() -> Self {
        let (a, b) = utils::generate_params();

        let mut rng = rand::thread_rng();
        let m: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();
        let r: Vec<u64> = (0..fft::LOCAL_N).map(|_| rng.gen_range(0..fft::LOCAL_Q)).collect();

        let commitment_point = RevelerCommit::new(a, b, m, r);
        let commitment_result = commitment_point.commit().unwrap_or_else(|err| {
            panic!("Commitment computation failed: {:?}", err);
        });

        commitment_result
    }
}


