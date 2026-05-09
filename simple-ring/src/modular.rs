
#[inline(always)]
pub fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 { //Function that computes b^e mod m, without overflowing.
    let mut result = 1u128;
    base %= modulus;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp >>= 1;
    }
    result
}

pub fn find_valid_omega(n: usize, q: u64) -> u64 { //Function that finds a valid omega for the parameters. 
    let two_n = 2 * n;
    let exp = (q - 1) / two_n as u64;
    
    for candidate in 2..q {
        let omega = mod_pow(candidate as u128, exp as u128, q as u128) as u64;
        
        let omega_n = mod_pow(omega as u128, n as u128 , q as u128);
        if omega_n == (q as u128 - 1)  {
            return omega;
        }
    }
    panic!("No valid omega found");
}

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

fn is_prime(n: u64) -> bool { //Function that return if an uint is prime or not.
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n.is_multiple_of(2) { return false; }
    let mut i = 3u64;
    while i * i <= n {
        if n.is_multiple_of(i) { return false; }
        i += 2;
    }
    true
}