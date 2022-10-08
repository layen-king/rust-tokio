use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;

/// 基准值
const BASE: usize = 42;
/// 使用线程数
const THREADS: usize = 8;

static DIFFICULTY: &'static str = "000000";
struct Solution(usize, String);

fn main() {
    println!(
        "Pow:Find a number, SHA256(the number * {}) == {}",
        BASE, DIFFICULTY
    );
    println!("Started {} threads", THREADS);
    println!("Please wait...");
    let start_time = Instant::now();
    // 全局原子
    let is_solution_found = Arc::new(AtomicBool::new(false));
    // 通道
    let (sender, receiver) = mpsc::channel();
    for i in 0..THREADS {
        // 复制发送者
        let sender_n = sender.clone();
        let is_solution_found = is_solution_found.clone();
        thread::spawn(move || find(i, sender_n, is_solution_found));
    }
    match receiver.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution:");
            println!(
                "The number is :{}, and hash result is :{}, target is:{}",
                i, hash, DIFFICULTY
            );
            let duration = start_time.elapsed();
            println!("Using time :{:?}", duration);
        }
        Err(_) => {
            panic!("Worker thread disconnected")
        }
    }
}

/// 校验数字是否有效
fn verify(number: usize) -> Option<Solution> {
    let mut hasher = Sha256::new();
    hasher.input_str(&(number * BASE).to_string());
    let hash = hasher.result_str();
    if hash.starts_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

/// 查找数字
fn find(start_at: usize, sender: mpsc::Sender<Solution>, is_solution_found: Arc<AtomicBool>) {
    // 根据进程无限步进
    for number in (start_at..).step_by(THREADS) {
        // 若全局的原子为true,表明其他线程已经找到,直接返回
        if is_solution_found.load(Ordering::Relaxed) {
            return;
        }
        // 若数字符合条件
        if let Some(solution) = verify(number) {
            // 修改全局
            is_solution_found.store(true, Ordering::Relaxed);
            // 通过通道发送
            sender.send(solution).unwrap();
            return;
        }
    }
}
