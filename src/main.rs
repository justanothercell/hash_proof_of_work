use std::thread;
use std::time::SystemTime;
use rand::Rng;
use sha3::{Digest, Sha3_256};

fn main() {
    let start = SystemTime::now();
    const INPUT_START: [u8; 8] = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
    const HASH_ENDING: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

    for i in 0..64 {
        let _ = thread::spawn(move || {
            loop {
                let random_bytes = rand::thread_rng().gen::<[u8; 24]>();
                let input: [u8; 32] = unsafe {
                    let mut result = std::mem::MaybeUninit::uninit();
                    let dest = result.as_mut_ptr() as *mut u8;
                    std::ptr::copy_nonoverlapping(INPUT_START.as_ptr(), dest, 8);
                    std::ptr::copy_nonoverlapping(random_bytes.as_ptr(), dest.add(8), 24);
                    result.assume_init()
                };
                let mut hasher = Sha3_256::new();
                hasher.update(input);
                let result = hasher.finalize();
                if result.ends_with(&HASH_ENDING) {
                    let in_string = input.map(|b|format!("{b:02x}")).join("");
                    let hash_string = result.into_iter().map(|b|format!("{b:02x}")).collect::<Vec<_>>().join("");
                    println!("[{:>9.3?}] thread[{i:02x}]: {in_string} -> {hash_string}", SystemTime::now().duration_since(start).unwrap())
                }
            }
        });
    }
    loop {}
}

/*
fn main() {
    let start = SystemTime::now();
    for i in 0..64 {
        let _ = thread::spawn(move || {
            loop {
                let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
                let mut hasher = Sha3_256::new();
                hasher.update(random_bytes);
                let result = hasher.finalize();
                if result.ends_with(&[0, 0, 0, 0]) {
                    let in_string = random_bytes.map(|b|format!("{b:x}")).join("");
                    let hash_string = result.into_iter().map(|b|format!("{b:x}")).collect::<Vec<_>>().join("");
                    println!("[{:?}] thread[{i}]: {in_string} -> {hash_string}", SystemTime::now().duration_since(start).unwrap())
                }
            }
        });
    }
    loop {}

 */