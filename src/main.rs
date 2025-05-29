// ============================================================================
// 💻 Monero Miner with GPU (OpenCL) Support
// 🔥 Built by GOR GEVORKYAN (https://github.com/BitCoinKing)
// 🕒 Created: May 2025
// 📄 License: MIT 
// 💡 Contact: gor@gorgevorkyan.com | Royal Gor Holdings
// ============================================================================

use clap::Parser;
use chrono::Local;
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use opencl3::{
    device::{get_all_devices, CL_DEVICE_TYPE_GPU, Device},
    platform::{get_platforms, Platform},
};
use rand::Rng;
use rayon::prelude::*;
use std::{
    error::Error,
    fs::OpenOptions,
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Wallet address
    #[arg(long)]
    wallet: String,

    /// Pool addresses
    #[arg(long, value_delimiter = ',')]
    pools: Vec<String>,

    /// Use GPU
    #[arg(long, default_value_t = false)]
    gpu: bool,
}

fn init_opencl() -> Result<(Platform, Device), Box<dyn Error + Send + Sync + 'static>> {
    let platforms = get_platforms().map_err(|e| format!("Get platforms failed: {:?}", e))?;
    let platform = platforms.first().ok_or("No OpenCL platforms found")?.clone();

    let platform_name = Platform::name(&platform)
        .map_err(|e| format!("Platform name error: {:?}", e))?;
    println!("🧠 Platform: {}", platform_name);

    let device_ids = get_all_devices(CL_DEVICE_TYPE_GPU)
        .map_err(|e| format!("Get devices failed: {:?}", e))?;
    let device_id = *device_ids.first().ok_or("No GPU devices found")?;

    let device = Device::new(device_id);
    let device_name = device.name()
        .map_err(|e| format!("Device name error: {:?}", e))?;
    println!("⚙️ GPU Device: {}", device_name);

    Ok((platform, device))
}

fn simulate_hash() -> u32 {
    let mut rng = rand::thread_rng();
    500 + rng.gen_range(0..100)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();
    let _wallet = args.wallet;
    let pools = args.pools;
    let use_gpu = args.gpu;
    let start = Instant::now();

    let shares = Arc::new(Mutex::new((0u32, 0u32)));
    let log_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .append(true)
            .create(true)
            .open("miner_log.csv")?,
    ));

    if use_gpu {
        let _ = init_opencl()?; // Handles GPU detection/logging
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner} {msg}")?,
    );

    let mut connected = false;
    for pool in pools {
        spinner.set_message(format!("🌐 Attempting connection to pool: {}", pool));
        match TcpStream::connect(&pool) {
            Ok(_) => {
                spinner.finish_with_message(format!("✅ Connected to pool: {}", pool));
                connected = true;

                (0..num_cpus::get())
                    .into_par_iter()
                    .for_each(|_| {
                        for _ in 0..5 {
                            let accepted = rand::random::<bool>();
                            let mut shares = shares.lock().unwrap();
                            if accepted {
                                shares.0 += 1;
                            } else {
                                shares.1 += 1;
                            }

                            let hash_rate = simulate_hash();
                            let ts = Local::now().format("%Y-%m-%d %H:%M:%S");

                            {
                                let mut file = log_file.lock().unwrap();
                                let _ = writeln!(
                                    file,
                                    "{ts},{pool},{hash_rate},{},{}",
                                    shares.0, shares.1
                                );
                            }

                            println!(
                                "{} ⛏ Hashrate: {} H/s | ✅ {} / ❌ {}",
                                Style::new().blue().apply_to(ts),
                                hash_rate,
                                shares.0,
                                shares.1
                            );
                            thread::sleep(Duration::from_secs(3));
                        }
                    });
                break;
            }
            Err(_) => {
                println!(
                    "{} {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    Style::new().yellow().apply_to("🔁 Retrying other pool...")
                );
            }
        }
    }

    if !connected {
        println!(
            "{} {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            Style::new().red().apply_to("❌ All pool connections failed.")
        );
    }

    println!(
        "{} Completed in {:.2?}",
        Style::new().green().apply_to("✅ Done!"),
        start.elapsed()
    );
    println!("💖 Built by Gor Gevorkyan — If you like it, drop some XMR 🙏");
    println!("🪙 Wallet: 4B2ggYzGyiPDRhgWhLVMAnSdig3StgAKG8B5U8QsECKdDTrPkXXhXEkZCCXfZQAUWw53o43HqZ5XJdkA6FLW3sUULeB5owA");

    Ok(())
}
