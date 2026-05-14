use simple_ring::is_prime;


pub fn find_valid_q(n: usize, t: u64, min_delta: u64) -> u64 { //Function that, given the difference between t and q, and n, searches for a valid q.
    let modulus = 2 * n as u64; // q ≡ 1 mod 2n
    let min_q = min_delta * t;
    
    let mut candidate = ((min_q / modulus) + 1) * modulus + 1;
    loop {
        if is_prime(candidate) {
            return candidate;
        }
        candidate += modulus;
    }
}