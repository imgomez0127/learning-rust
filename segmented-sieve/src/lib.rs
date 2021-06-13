use std::cmp;
use std::vec::Vec;
use core::slice::Iter;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::mem::drop;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

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


fn classic_sieve(sqrtn:u64) -> Vec<u64> {
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


#[pyfunction]
fn sieve(max_val:u64, size:u64, num_threads:usize) -> PyResult<Vec<u64>> {
    let blocks = get_blocks(max_val, size);
    let sqrtn = (max_val as f64).sqrt() as u64;
    let primes = Arc::new(classic_sieve(sqrtn as u64));
    let all_prime_counter = Arc::new(Mutex::new(Vec::new()));
    let block_nums: Vec<u64> = (0..blocks).collect();
    let block_nums_counter = Arc::new(Mutex::new(block_nums));
    let mut handles = vec![];
    for _ in 0..num_threads {
        let all_prime_mutex = Arc::clone(&all_prime_counter);
        let block_nums_mutex = Arc::clone(&block_nums_counter);
        let primes_ref = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            let mut blocks_guard = block_nums_mutex.lock().unwrap();
            let mut block_opt = (*blocks_guard).pop();
            drop(blocks_guard);
            while !block_opt.is_none(){
                let block = block_opt.unwrap();
                let new_primes = segmented_sieve(block, size, primes_ref.iter());
                let mut all_primes = all_prime_mutex.lock().unwrap();
                (*all_primes).extend(new_primes.iter().copied());
                blocks_guard = block_nums_mutex.lock().unwrap();
                block_opt = (*blocks_guard).pop();
                drop(blocks_guard);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let all_primes_mutex = Arc::clone(&all_prime_counter);
    let mut all_primes = all_primes_mutex.lock().unwrap();
    (*all_primes).sort();
    let mut result = Vec::new();
    for prime in all_primes.iter() {
        result.push(*prime)
    }
    Ok(result)
}


#[pymodule]
fn concurrent_sieve(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sieve, m)?)?;
    Ok(())
}
