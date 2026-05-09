#[cfg(feature = "parallel")]
use rayon::prelude::*;          
#[cfg(feature = "parallel")]
use rayon::slice::ParallelSliceMut;

use crate::RingParams;
use crate::modular::mod_pow;
use crate::polys::Polynomial;

#[derive(Clone)]
pub struct NTTprecaculated {
    twiddles: Box<[u128]>,
    twiddles_inv: Box<[u128]>,
}



pub fn precalculate(params: &RingParams) -> NTTprecaculated {
    let n = params.n;
    let q = params.q as u128;
    let psi = params.omega as u128;  
    let log_n = n.trailing_zeros();
   
    let mut twiddles = vec![1u128; n];
    for k in 0..n {
        let br_k = k.reverse_bits() >> (64 - log_n);
        let power = br_k as u128;        
        twiddles[k] = mod_pow(psi, power, q);
    }
    
  
    let psi_inv = mod_pow(psi, q - 2, q);
    let mut twiddles_inv = vec![1u128; n];
    for k in 0..n {
        let br_k = k.reverse_bits() >> (64 - log_n);
        let power = br_k as u128;
        twiddles_inv[k] = mod_pow(psi_inv, power, q);
    }
    
    NTTprecaculated {
        twiddles: twiddles.into_boxed_slice(),
        twiddles_inv: twiddles_inv.into_boxed_slice(),
    }
}


#[cfg(not(feature = "parallel"))]
pub fn forward_ntt(params: &RingParams, polynomial: &Polynomial, ntt_tables: &NTTprecaculated) -> Polynomial {
    forward_ntt_single( params,  polynomial, ntt_tables)
}

#[cfg(not(feature = "parallel"))]
pub fn inverse_ntt(params: &RingParams, polynomial: &Polynomial, ntt_tables: &NTTprecaculated) -> Polynomial {
    inverse_ntt_single( params,  polynomial, ntt_tables)
}

#[cfg(feature = "parallel")]
pub fn forward_ntt(params: &RingParams, polynomial: &Polynomial, ntt_tables: &NTTprecaculated) -> Polynomial {
    forward_ntt_multi( params,  polynomial, ntt_tables)
}

#[cfg(feature = "parallel")]
pub fn inverse_ntt(params: &RingParams, polynomial: &Polynomial, ntt_tables: &NTTprecaculated) -> Polynomial {
    inverse_ntt_multi( params,  polynomial, ntt_tables)
}



fn forward_ntt_single(
    params: &RingParams,
    polynomial: &Polynomial,
    ntt_tables: &NTTprecaculated
) -> Polynomial {
    let mut coeffs: Vec<u64> = polynomial.coeffs.to_vec();
        

    let n = params.n;
    let q = params.q as u128;
    let mut t: usize = n;
    let mut m: usize = 1;

    loop {
        t /= 2;

        
        for i in 0..m {  
            let j1 = 2 * i * t;
            let j2 = j1 + t - 1;

            for j in j1..=j2 {
                let u = coeffs[j] as u128;
                let w = ntt_tables.twiddles[m + i]; 
                let v = ((coeffs[j + t]) as u128 * w) % q;

                    let mut x = u + v;
                    if x >= q { x -= q; }
                    coeffs[j] = x as u64;

                    let y = if u >= v { u - v } else { u + q - v };
                    coeffs[j + t] = y as u64;

            }
        }

        m *= 2;

        if m >= n { break; }
    }

    Polynomial {
        coeffs: coeffs.into_iter().collect::<Vec<_>>().into_boxed_slice(),
    }
}

fn inverse_ntt_single(
    params: &RingParams,
    polynomial: &Polynomial,
    ntt_tables: &NTTprecaculated
) -> Polynomial {
    let mut coeffs: Vec<u64> = polynomial.coeffs.to_vec();

    let n = params.n;
    let q = params.q as u128;

    let mut t: usize = 1;
    let mut m: usize = n;

    loop {
        let h = m / 2;
        let mut j1: usize = 0;

        for i in 0..h {
            let j2 = j1 + t - 1;

            for j in j1..=j2 {
                let u = coeffs[j] as u128;
                let v = coeffs[j + t] as u128;

                let sum = u + v;
                let sum = if sum >= q { sum - q } else { sum };
                coeffs[j] = sum as u64;                

                
                let w = ntt_tables.twiddles_inv[h + i];
                coeffs[j + t] = (((u + q - v) % q * w) % q) as u64;
            }

            j1 += 2 * t;
        }

        t *= 2;
        m /= 2;

        
        if m <= 1 { break; }
    }

    let n_inv = mod_pow(n as u128, q - 2, q);
    for c in coeffs.iter_mut() {
        *c = (*c * n_inv as u64) % q as u64;
    }

    Polynomial {
        coeffs: coeffs.into_iter().collect::<Vec<_>>().into_boxed_slice(),
    }
}


#[cfg(feature = "parallel")]
fn forward_ntt_multi(
    params: &RingParams,
    polynomial: &Polynomial,
    ntt_tables: &NTTprecaculated
) -> Polynomial {
    let mut coeffs: Vec<u128> = polynomial.coeffs
        .iter()
        .map(|&c| c as u128)
        .collect();

    let n = params.n;
    let q = params.q as u128;

    let mut t: usize = n;
    let mut m: usize = 1;

    loop {
        t = t / 2;

       

    coeffs
    .par_chunks_mut(2 * t)
    .enumerate()
    .for_each(|(i, chunk)| {
        for j in 0..t {
            let u = chunk[j];
            let w = ntt_tables.twiddles[m + i];
            let v = ((chunk[j + t]) * w) % params.q as u128;
            let sum = u + v;
           
            chunk[j] = if sum >= q { sum - q } else { sum };

            chunk[j + t] = if u >= v { u - v } else { u + q - v };
        }
    });
   

        m = m * 2;

        if m >= n { break; }
    }

    Polynomial {
        coeffs: coeffs.into_iter().map(|c| c as u64).collect::<Vec<_>>().into_boxed_slice(),
    }
}

#[cfg(feature = "parallel")]
fn inverse_ntt_multi(
    params: &RingParams,
    polynomial: &Polynomial,
    ntt_tables: &NTTprecaculated
) -> Polynomial {
    let mut coeffs: Vec<u128> = polynomial.coeffs
        .iter()
        .map(|&c| c as u128)
        .collect();

    let n = params.n;
    let q = params.q as u128;

    let mut t: usize = 1;
    let mut m: usize = n;

    loop {
        let h = m / 2;

    coeffs
    .par_chunks_mut(2 * t)
    .enumerate()
    .for_each(|(i, chunk)| {
        let w = ntt_tables.twiddles_inv[h + i];

        for j in 0..t {
            let u = chunk[j];
            let v = chunk[j + t];

            let sum = u + v;
            chunk[j] = if sum >= q { sum - q } else { sum };

            chunk[j + t] = ((u + q - v) % q * w) % q;
        }
    });

        t = t * 2;
        m = m / 2;

       
        if m <= 1 { break; }
    }

    let n_inv = mod_pow(n as u128, q - 2, q);
    for c in coeffs.iter_mut() {
        *c = (*c * n_inv) % q;
    }

    Polynomial {
        coeffs: coeffs.into_iter().map(|c| c as u64).collect::<Vec<_>>().into_boxed_slice(),
    }
}