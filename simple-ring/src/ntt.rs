#[cfg(feature = "parallel")]
use rayon::prelude::*;          
#[cfg(feature = "parallel")]
use rayon::slice::ParallelSliceMut;

use crate::RingParams;
use crate::modular::mod_pow;
use crate::poly::Polynomial;
#[cfg(not(feature = "parallel"))]
use crate::traits::RingCompatible;

#[derive(Clone)]
pub struct NTTprecaculated {
    pub twiddles: Box<[u128]>,
    pub twiddles_inv: Box<[u128]>,
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
pub fn forward_ntt<T: RingCompatible + Clone>(params: &RingParams, polynomial: &Polynomial<T>, ntt_tables: &NTTprecaculated) -> Polynomial<T> {
    forward_ntt_single( params,  polynomial, ntt_tables)
}

#[cfg(not(feature = "parallel"))]
pub fn inverse_ntt<T: RingCompatible + Clone>(params: &RingParams, polynomial: &Polynomial<T>, ntt_tables: &NTTprecaculated) -> Polynomial<T> {
    inverse_ntt_single( params,  polynomial, ntt_tables)
}

#[cfg(feature = "parallel")]
pub fn forward_ntt<T: RingCompatible + Clone>(params: &RingParams, polynomial: &Polynomial<T>, ntt_tables: &NTTprecaculated) -> Polynomial<T> {
    forward_ntt_multi( params,  polynomial, ntt_tables)
}

#[cfg(feature = "parallel")]
pub fn inverse_ntt<T: RingCompatible + Clone>(params: &RingParams, polynomial: &Polynomial<T>, ntt_tables: &NTTprecaculated) -> Polynomial<T> {
    inverse_ntt_multi( params,  polynomial, ntt_tables)
}



fn forward_ntt_single<T: RingCompatible + Clone>(
    params: &RingParams,
    polynomial: &Polynomial<T>,
    ntt_tables: &NTTprecaculated
) -> Polynomial<T> {
    let mut coeffs: Vec<T> = polynomial.to_vec();
        

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
                let u = coeffs[j].to_u128();
                let w = ntt_tables.twiddles[m + i]; 
                let v = ((coeffs[j + t]).to_u128() * w) % q;

                    let mut x = u + v;
                    if x >= q { x -= q; }
                    coeffs[j] = T::from_u128(x);

                    let y = if u >= v { u - v } else { u + q - v };
                    coeffs[j + t] = T::from_u128(y);

            }
        }

        m *= 2;

        if m >= n { break; }
    }

    Polynomial::new_from_coeffs(coeffs)
}

fn inverse_ntt_single<T: RingCompatible + Clone>(
    params: &RingParams,
    polynomial: &Polynomial<T>,
    ntt_tables: &NTTprecaculated
) -> Polynomial<T> {
    let mut coeffs: Vec<T> = polynomial.to_vec();

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
                let u = coeffs[j].to_u128();
                let v = coeffs[j + t].to_u128();

                let sum = u + v;
                let sum = if sum >= q { sum - q } else { sum };
                coeffs[j] = T::from_u128(sum);                

                
                let w = ntt_tables.twiddles_inv[h + i];
                coeffs[j + t] = T::from_u128(((u + q - v) % q * w) % q);
            }

            j1 += 2 * t;
        }

        t *= 2;
        m /= 2;

        
        if m <= 1 { break; }
    }

    let n_inv = mod_pow(n as u128, q - 2, q);

    for c in coeffs.iter_mut() {
        *c = T::from_u128((c.to_u128() * n_inv) % q as u128);
    }

    Polynomial::new_from_coeffs(coeffs)
}


#[cfg(feature = "parallel")]
fn forward_ntt_multi<T: RingCompatible + Clone>(
    params: &RingParams,
    polynomial: &Polynomial<T>,
    ntt_tables: &NTTprecaculated
) -> Polynomial {
    let mut coeffs: Vec<T> = polynomial.to_vec();


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
            let v = ((chunk[j + t].to_u128()) * w) % params.q as u128;
            let sum = u + v;
           
            chunk[j] = if sum >= q { T::from_u128(sum - q) } else { T::from_u128(sum) };

            chunk[j + t] = if u >= v { T::from_u128(u - v) } else { T::from_u128(u + q - v) };
        }
    });
   

        m = m * 2;

        if m >= n { break; }
    }

    Polynomial::new_from_coeffs(coeffs)
}

#[cfg(feature = "parallel")]
fn inverse_ntt_multi<T: RingCompatible + Clone>(
    params: &RingParams,
    polynomial: &Polynomial,
    ntt_tables: &NTTprecaculated
) -> Polynomial {
    let mut coeffs: Vec<u128> = polynomial.to_vec();

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
            let u = chunk[j].to_u128();
            let v = chunk[j + t].to_u128();

            let sum = u + v;
            chunk[j] = if sum >= q { T::from_u128(sum - q) } else { T::from_u128(sum) };

            chunk[j + t] = ((u + q - v) % q * w) % q;
        }
    });

        t = t * 2;
        m = m / 2;

       
        if m <= 1 { break; }
    }

    let n_inv = mod_pow(n as u128, q - 2, q);
    for c in coeffs.iter_mut() {
        *c = T::from_u128((c.to_u128() * n_inv) % q);
    }

    Polynomial::new_from_coeffs(coeffs)
}