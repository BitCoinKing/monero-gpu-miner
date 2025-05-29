# 🧠 Monero GPU Miner (OpenCL-Powered)

High-performance, async Rust-based GPU miner for Monero (XMR), built with OpenCL support.

> ⚡ Created by **GOR GEVORKYAN**  
> 🏢 DEV 101 LABS  
> 🌐 https://github.com/BitCoinKing  
> 💸 Donate: `4B2ggYzGyiPDRhgWhLVMAnSdig3StgAKG8B5U8QsECKdDTrPkXXhXEkZCCXfZQAUWw53o43HqZ5XJdkA6FLW3sUULeB5owA`

---

### 🛠 Features

- 🚀 GPU mining via OpenCL (Apple M-series compatible)
- 🔁 Multi-pool fallback logic
- 📊 Real-time hashrate output
- 💾 Logging via `miner_log.csv`
- 📦 Built in Rust for max performance
- 💬 Clean terminal output

---

### 🚀 Run It

```bash
cargo run -- \
  --wallet 4B2ggYzGyiPDRhgWhLVMAnSdig3StgAKG8B5U8QsECKdDTrPkXXhXEkZCCXfZQAUWw53o43HqZ5XJdkA6FLW3sUULeB5owA \
  --pools pool.supportxmr.com:3333 \
  --gpu
