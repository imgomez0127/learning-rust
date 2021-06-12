use std::io;
use std::cmp;
use std::vec::Vec;
use core::slice::Iter;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::mem::drop;

fn segmented_sieve(block:u64, size:u64, primes:Iter<u64>) -> Vec<u64> {
    let mut is_prime = Vec::new();
    let mut new_primes = Vec::new();
    for _ in 0..size {
        is_prime.push(true)
    }
    let offset = block*size;
    for prime in primes {
        let start_idx = (offset+prime-1)/prime;
        let start_val = cmp::max(start_idx, *prime) * prime - offset;
        for j in (start_val..size).step_by(*prime as usize) {
            let ind = j as usize;
            is_prime[ind] = false;
        }
    }
    if block == 0 {
        is_prime[0] = false;
        is_prime[1] = false;
    }
    for i in 0..is_prime.len() {
        if is_prime[i] {
            let prime_val = i as u64;
            new_primes.push(prime_val + offset);
        }
    }
    return new_primes;
}


fn sieve(sqrtn:u64) -> Vec<u64> {
    let mut is_prime = Vec::new();
    let mut primes = Vec::new();
    for _ in 0..=sqrtn {
        is_prime.push(true);
    }
    is_prime[0] = false;
    is_prime[1] = false;
    let sqrtsqrtn = (sqrtn as f64).sqrt() as u64;
    for i in 0..=sqrtsqrtn {
        if is_prime[i as usize] {
            for j in (i*i..=sqrtn).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    for i in 0..is_prime.len() {
        if is_prime[i]  {
            primes.push(i as u64);
        }
    }
    return primes;
}


fn get_blocks(max_val:u64, size:u64) -> u64 {
    if max_val%size == 0 {
        let blocks = max_val/size;
        return blocks;
    } else {
        let blocks = max_val/size + 1;
        return blocks;
    }
}

fn concurrent_sieve(max_val:u64, size:u64, num_threads:usize) -> Vec<u64> {
    let blocks = get_blocks(max_val, size);
    let sqrtn = (max_val as f64).sqrt() as u64;
    let primes = sieve(sqrtn as u64);
    let all_prime_counter = Arc::new(Mutex::new(Vec::new()));
    let block_nums_counter = Arc::new(Mutex::new((0..num_threads).collect()));
    let handles = Vec::new();
    for _ in 0..num_threads {
        let all_prime_mutex = Arc::clone(&all_prime_counter);
        let block_nums_mutex = Arc::clone(&block_nums_counter);
        let handle = thread::spawn(move || {
            let blocks_guard = block_nums_mutex.lock().unwrap();
            let block = (*blocks_guard).pop();
            drop(blocks_guard);
            let new_primes = segmented_sieve(block, size, primes.iter());
            let all_primes = all_prime_mutex.lock().unwrap();
            *all_primes.extend(new_primes.iter().copied());
        });
        handles.push(handle);
    }
    for handle in handles.iter() {
        handle.join().unwrap();
    }
    let all_primes_mutex = Arc::clone(&all_prime_counter);
    let all_primes = all_primes_mutex.lock().unwrap();
    *all_primes.sort();
    return *all_primes;
}

fn main() {
    let mut input = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    println!("Amount of primes to compute");
    let _result = io::stdin().read_line(&mut input);
    let max_val = input.split_whitespace().next().unwrap().parse::<u64>().unwrap();
    println!("Size of chunks");
    let _result2 = io::stdin().read_line(&mut input2);
    let size = input2.split_whitespace().next().unwrap().parse::<u64>().unwrap();
    let _result3 = io::stdin().read_line(&mut input3);
    let num_threads = input3.split_whitespace().next().unwrap().parse::<usize>().unwrap();
    println!("Max val to compute: {}", max_val);
    println!("Chunk size: {}", size);
    println!("Num Threads {}", size);
    let all_primes = concurrent_sieve(max_val, size, num_threads);
    for prime in all_primes.iter() {
        println!("{ }", prime);
    }
}
