
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} -N <zeros> -F <count>", args[0]);
        return;
    }

    let n: usize = args[2].parse().expect("Invalid number of zeros");
    let f: usize = args[4].parse().expect("Invalid number of results");

    let zeros = "0".repeat(n);
    let found = Arc::new(AtomicUsize::new(0));

    (1u64..u64::MAX).into_par_iter().for_each(|i| {
        if found.load(Ordering::SeqCst) >= f {
            return;
        }

        let mut hasher = Sha256::new();
        hasher.update(i.to_string());
        let hash = format!("{:x}", hasher.finalize());

        if hash.ends_with(&zeros) {
            let current = found.fetch_add(1, Ordering::SeqCst);
            if current < f {
                println!("{}, \"{}\"", i, hash);
            }
        }
    });
}
