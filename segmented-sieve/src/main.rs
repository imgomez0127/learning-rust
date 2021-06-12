use std::io
use std::cmp
use std::num::Float;
use std::convert::FloatToInt

fn segmented_sieve(block:u64, size:u64, sqrtn:u64, primes:Vec<u64>) {
    let mut Vec<bool> is_prime = Vec::with_capacity(size);
    let mut Vec<u64> new_primes = Vec::new();
    is_prime.fill(true);
    let offset = block*size;
    for prime in primes.iter() {
        let start_idx = (offset+prime-1)/offset;
        let start_val = cmp::max(start_idx, p) * p - offset;
        for j in (start_val..=S).skip_by(prime) {
            is_prime[j] = false;
        }
    }
    if(block == 0){
        is_prime[0] = false;
        is_prime[1] = false;
    }
    for i in 0..new_primes.len() {
        if(is_prime[i]) {
            new_primes.append(i);
        }
    }
    return new_primes;
}

fn sieve(sqrtn:u64) {
    let mut Vec<bool> is_prime = Vec::with_capacity(sqrtn+1);
    let mut Vec<u64> primes = Vec::new();
    is_prime.fill(true);
    is_prime[0] = false;
    is_prime[1] = true;
    let sqrtsqrtn = FloatToInt<u64>sqrtn(sqrtn);
    for i in 0..=sqrtsqrtn {
        if(is_prime[i]){
            for j in (i*i..=sqrtn).skip_by(i) {
                is_prime[j] = false;
            }
        }
    }
    for i in 0..is_prime.len() {
        if(is_prime[i]) {
            primes.append(i);
        }
    }
    return primes;
}

fn main() {
    let mut input = String::new();
    println!("Amount of primes to compute")
    io::stdin.read_line(&mut input);
    let max_val = input.parse::<u64>().unwrap();
    let sqrtn = FloatToInt<u64>(sqrt(max_val));
    println!("Size of chunks")
    io::stdin.read_line(&mut input);
    let size = input.parse::<u64>.unwrap();
    if(max_val%size == 0){
        let blocks = max_val/size;
    } else {
        let blocks = (max_val/size)+1;
    }
    let primes = sieve(sqrtn);
    let all_primes = Vec<u64>::new();
    for block in 0..=blocks {
        all_primes.extend(segmented_sieve(block, size, primes).iter().copied());
    }
    for prime in all_primes.iter() {
        println!("{ }", prime);
    }
}
