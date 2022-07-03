use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::sync::{mpsc, RwLock};
use std::thread;
use once_cell::sync::Lazy;

/// 全局锁
static IS_SOLUTION_FOUND:Lazy<RwLock<bool>> = Lazy::new(||{
  RwLock::new(false)
});

/// 基准值
const BASE: usize = 42;
/// 使用线程数
const THREADS: usize = 8;

static DIFFICULTY: &'static str = "0000";
struct Solution(usize, String);

fn main() {
    println!(
        "Pow:Find a number, SHA256(the number * {}) == {}",
        BASE, DIFFICULTY
    );
    println!("Started {} threads", THREADS);
    println!("please wait...");
    // 通道
    let (sender, receiver) = mpsc::channel();
    for i in 0..THREADS {
        // 复制发送者
        let sender = sender.clone();
        thread::spawn(move || find(i, sender));
    }
    match receiver.recv() {
        Ok(Solution(i, hash)) => {
            println!("Found the solution:");
            println!("the number is :{}, and hash result is :{}", i, hash);
        }
        Err(_) => {
            panic!("worker thread disconnected")
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
fn find(start_at: usize, sender: mpsc::Sender<Solution>) {
    // 根据进程无限步进
    for number in (start_at..).step_by(THREADS) {
        // 若全局的原子为true,表明其他线程已经找到,直接返回
        if *IS_SOLUTION_FOUND.read().unwrap() {
            return;
        }
        // 若数字符合条件
        if let Some(solution) = verify(number) {
            // 修改全局
            *IS_SOLUTION_FOUND.write().unwrap() = true;
            // 通过通道发送
            sender.send(solution).unwrap();
            return;
        }
    }
}
