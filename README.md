# Reveler - [中文文档](https://github.com/blueokanna/reveler/blob/main/README-zh.md)

This repository implements a cryptographic commitment scheme based on optimized FFT and hashing functions, designed for efficient commitment generation and verification. The scheme uses Fast Fourier Transform (FFT) for matrix multiplication and BlueHash-based hashing for commitment verification.


## Overview

This library provides two main functions:
- **Commitment Generation** (`commit`): This function generates a cryptographic commitment based on input parameters.
- **Commitment Verification** (`verify`): This function verifies the validity of a commitment using a random challenge.

The cryptographic commitment uses a combination of FFT-based matrix multiplication and a multi-round BlueHash hashing algorithm to ensure both randomness and security.

## Commitment Generation

The `commit` function computes the commitment using the following steps:

1. **Matrix Generation**: Random matrices `A` and `B` are generated using the `generate_params` function. These matrices will be used in the FFT-based matrix multiplication.

   The matrices \( A \) and \( B \) are of size \( N \times N \), where \( N = 256 \). Each element is randomly chosen from the range \( [0, Q) \), where \( Q = 65535 \).

   ![p1](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p1.jpg)

2. **FFT Matrix Multiplication**: For each row \( a_i \) from matrix \( A \) and \( b_i \) from matrix \( B \), we perform an FFT-based matrix multiplication:

    ![p2](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p2.jpg)

   Where represents element-wise multiplication of the FFT-transformed rows and the vectors \( m \) and \( r \), respectively.

3. **Commitment Point Calculation**: The sum of the FFT results for each row is calculated as:

   ![p3](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p3.jpg)

   **This results in a commitment point:**
   
   ![p4](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p4.jpg)

5. **Commitment Hashing**: The commitment point \( C \) is then hashed using the BlueHash algorithm. The BlueHash algorithm applies multiple rounds of hashing for added randomness and security:

   ![p5](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p5.jpg)

   The final commitment hash is obtained after 3 rounds of BlueHash.

   The commitment is returned as a struct containing both the commitment point \( C \) and its hash \( H(C) \).

## Verification

The `verify` function checks the validity of a commitment. It recomputes the commitment hash from the commitment point and compares it to the stored commitment hash:

![p6](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p6.jpg)

Where \( C' \) is the commitment point being verified. If the recomputed hash \( H(C') \) matches the original hash, the commitment is valid.

## Mathematical Foundations

### Fast Fourier Transform (FFT) for Matrix Multiplication

The commitment generation relies heavily on FFT-based matrix multiplication. Given two matrices \( A \) and \( B \), the rows of these matrices are transformed into frequency space using FFT. The transformation is given by:

![p7](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p7.jpg)

Where \( \mathcal{F} \) represents the FFT transformation.

The matrix multiplication in frequency space is done by element-wise multiplication of the FFT results, which is computationally more efficient than directly multiplying the matrices in the time domain.

### BlueHash Algorithm

The BlueHash algorithm is used for hashing the commitment point to generate the final commitment hash. It applies multiple rounds of hashing to increase the entropy and randomness of the hash.

![p5](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p5.jpg)

Where \( H(C) \) is the final hash after 3 rounds of the BlueHash algorithm.

## Code Structure

The project consists of three main modules:

1. **fft**: Implements FFT-based matrix multiplication for commitment generation.
   - Functions: `fft_matrix_multiply`
2. **utils**: Contains utility functions for random number generation, matrix creation, and BlueHash-based hashing.
   - Functions: `get_optimal_thread_count`, `hash_to_commitment`, `generate_params`
3. **commitment**: Implements the commitment structure and the core `commit` and `verify` functions.
   - Functions: `commit`, `verify`, `RevelerCommit`

### Example Code

```rust
fn main() {
    let (a, b) = generate_params();
    let mut rng = rand::thread_rng();
    let m: Vec<u64> = (0..N).map(|_| rng.gen_range(0..Q)).collect();
    let r: Vec<u64> = (0..N).map(|_| rng.gen_range(0..Q)).collect();

    let commitment = commit(&a, &b, &m, &r);
    println!("Commitment: {:?}", commitment);

    let is_valid = verify(&commitment);
    println!("Is commitment valid? {}", is_valid);
}
```


## Donations
| ![Tether](https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/16/tether.png "Tether (USDT)") **USDT** : Arbitrum One Network: **0x4051d34Af2025A33aFD5EacCA7A90046f7a64Bed** | ![USD Coin](https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/16/usd-coin.png "USD Coin (USDC)") **USDC**: Arbitrum One Network: **0x4051d34Af2025A33aFD5EacCA7A90046f7a64Bed** | ![Dash Coin](https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/16/dash.png "Dash Coin (Dash)") **Dash**: Dash Network: **XuJwtHWdsYzfLawymR3B3nDdS2W8dHnxyR** |
|------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|

| ![0x4051d34Af2025A33aFD5EacCA7A90046f7a64Bed](https://github.com/user-attachments/assets/608c5e0d-edfc-4dee-be6f-63d40b53a65f) | ![0x4051d34Af2025A33aFD5EacCA7A90046f7a64Bed (1)](https://github.com/user-attachments/assets/87205826-1f76-4724-9734-3ecbfbfb729f) | ![XuJwtHWdsYzfLawymR3B3nDdS2W8dHnxyR](https://github.com/user-attachments/assets/71915604-cc14-426f-a8b9-9b7f023da084) |
|------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
