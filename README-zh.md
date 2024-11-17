# Reveler - [English Version]()

该存储库基于优化的 FFT 和散列函数实现了一种加密承诺方案，旨在高效地生成和验证承诺。该方案使用快速傅立叶变换 (FFT) 进行矩阵乘法，使用基于 BlueHash 的散列进行承诺验证。


### 概览

该库提供两个主要功能：
- **承诺生成** (`commit`)： 该函数根据输入参数生成加密承诺。
- **承诺验证** (`verify`)： 该函数使用随机挑战验证承诺的有效性。

加密承诺结合使用了基于 FFT 的矩阵乘法和多轮 BlueHash 哈希算法，以确保随机性和安全性。

## 生成承诺

承诺 "函数通过以下步骤计算承诺：

1. 生成**矩阵**： 使用 `generate_params` 函数生成随机矩阵 `A` 和 `B`。这些矩阵将用于基于 FFT 的矩阵乘法。

   矩阵 \( A \) 和 \( B \) 的大小为 \( N \times N \)，其中 \( N = 256 \)。每个元素都是从范围 \( [0, Q) \)中随机选择的，其中 \( Q = 65535 \)。

   ![p1](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p1.jpg)

2. **FFT矩阵乘法**： 对于矩阵（A）中的每一行（a_i \）和矩阵（B）中的每一行（b_i \），我们执行基于 FFT 的矩阵乘法：

![p2](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p2.jpg)

其中分别表示 FFT 变换后的行与矢量 \( m \)和 \( r \)的元素相乘。

3. **承诺点计算**： 每一行的 FFT 结果之和的计算公式为

![p3](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p3.jpg)

由此得出一个承诺点 ![p4](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p4.jpg) 。

4. **承诺哈希**： 然后使用 BlueHash 算法对承诺点 \( C \) 进行散列。BlueHash 算法采用多轮散列，以增加随机性和安全性：

![p5](https://raw.githubusercontent.com/blueokanna/reveler/refs/heads/main/image/p5.jpg)

经过 3 轮 BlueHash 算法散列后，就得到了最终的承诺散列值。

承诺以结构体的形式返回，其中包含承诺点（C）和哈希值（H(C) ）。


### 代码结构

该项目由三个主要模块组成：

1. **fft**： 实现基于 FFT 的矩阵乘法，用于生成承诺。
- 函数： `fft_matrix_multiply`
2. **utils**： 包含随机数生成、矩阵创建和基于 BlueHash 的散列等实用功能。
- 函数： `get_optimal_thread_count`、`hash_to_commitment`、`generate_params`。
3. **commitment**： 实现承诺结构以及核心的 `commit` 和 `verify` 函数。
- 函数： `commit`、`verify`、`RevelerCommit`。

### 示例代码

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
