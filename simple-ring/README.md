# `simple-ring` : Simple implementation of mathematical primitives for LWE-based crypto in Rust

`simple-ring` provides mathematical primitives, such as NTT or Polynomials, but also coefficient sampling for LWE-based schemes. For more informations and explanations, please see `/docs/simple-ring.pdf`.

## Warning 

This code isn’t audited, isn’t intended for production use, and is vulnerable to timing-based attacks.

## Quick start

To use it, simply do :
```bash
cargo add simple-ring
```

### Examples 

#### Using the 'basic' polynomials

You can use `simple-ring` to manipulate **Polynomials** as wanted. For example :

```rust
use simple_ring::*;

fn main() {
    let poly_p = poly![-5, 3, 2];
    let poly_q = poly![2, -4, 1];

    println!("P(2) = {}", poly_p.eval(2));
    
    println!("Q(-1) = {}", poly_q.eval(-1));

    println!("A(x) = P(x) + Q(x) = {:?}", poly_p.clone() + poly_q.clone());

    println!("B(x) = P(x) - Q(x) = {:?}", poly_p.clone() - poly_q);
    
    println!("C(x) = P(x) * 10 = {:?}", poly_p * 10);
}
```

It displays :

```bash
P(2) = 9
Q(-1) = 7
A(x) = P(x) + Q(x) = Polynomial { coeffs: [-3, -1, 3] }
B(x) = P(x) - Q(x) = Polynomial { coeffs: [-7, 7, 1] }
C(x) = P(x) * 10 = Polynomial { coeffs: [-50, 30, 20] }
```

#### Using the NTT multiplication and computation in the ring

But `simple-ring` also provides a specific codebase for manipulating **Polynomials** in **ℤ𝑞 [𝑋]/(𝑋𝑛 + 1)**, including the NTT. For example :

```rust
use std::time::Instant;

use simple_ring::*;

fn main() {
    // Define the ring 
    let n = 4096;

    // Yep, this is huge. To validate q, you can use `is_q_valid()`
    let q = 512032769;

    let params = &RingParams::new(4096, q, find_valid_omega(n, q) );

    // Precompute the NTT tables 
    let ntt_tables = &precalculate(params);

    // Generate a polynomial with a random definition, in the ring
    let poly_1 = generate_uniform_polynomial::<u64>(params);

    
    // Create a polynomial of `2`
    let poly_2 = Polynomial::new_from_coeffs(vec![2u64; n]);


    // Compute naively the product of poly_1 and poly_2

    let start = Instant::now();

    let product = poly_1.ring_mul(params, &poly_2);

    let end = Instant::now();

    let elapsed = end - start;

    println!("Computed P1(x) * P2(x) in {} ms with naive mul", elapsed.as_millis());

    // Compute using ntt

    let start = Instant::now();

    let product_with_ntt = poly_1.mul_ntt(params, ntt_tables, &poly_2);

    let end = Instant::now();

    let elapsed = end - start;

    println!("Computed P1(x) * P2(x) in {} ms with NTT mul", elapsed.as_millis());

    assert_eq!(product.to_vec(), product_with_ntt.to_vec());
}
```

And it displays : 

```bash
Computed P1(x) * P2(x) in 195 ms with naive mul
Computed P1(x) * P2(x) in 2 ms with NTT mul
```

> [!NOTE]
> NTT really makes the difference on huge products

#### Using directly the NTT, and built-in helpers 

```rust
use std::time::Instant;

use simple_ring::{RingParams, find_valid_omega, forward_ntt, inverse_ntt, mod_pow, precalculate, sampling::generate_cbd_sample};

fn main() {
    // Define the ring 
    let n = 4096;

    // Yep, this is huge. To validate q, you can use `is_q_valid()`
    let q = 512032769;

    let params = &RingParams::new(n, q, find_valid_omega(n, q) );

    // Precompute the NTT tables 
    let ntt_tables = &precalculate(params);
    
    // A sample generated using `Centered Binomial Distribution`
    let cbd_sample = generate_cbd_sample(n, 8);

    // We pass the cbd_sample in the ring
    let poly = cbd_sample.to_poly_64(q);

    // We convert the polynomial in NTT domain
    let poly_in_ntt_domain = forward_ntt(params, &poly, ntt_tables);

    assert!(poly.to_vec() != poly_in_ntt_domain.to_vec());

    // We pass back in the ring ('normal' domain)
    let poly_back_in_ring = inverse_ntt(params, &poly_in_ntt_domain, ntt_tables);

    // Coefficients are the same
    assert_eq!(poly.to_vec(), poly_back_in_ring.to_vec());

    // I don't know why, but I want to compute `1546^8889 mod q`
    //
    // Doing it naively would be to heavy...

    let start = Instant::now();

    let result = mod_pow(1546, 8889, q as u128);

    let end = Instant::now();

    let elapsed = end - start;

    println!("1546^8889 mod {} = {} ; Computed in {} ns.", q, result, elapsed.as_nanos())
}
```

And it displays :

```bash
1546^8889 mod 512032769 = 220811177 ; Computed in 240 ns.
```

## Tests & modifications

To run the tests, or modify the code, you can git clone the repo, and then run the tests or do any modification you want :
```bash
cargo test --release -- --nocapture
```

## Repository

`simple-ring` is a crate from [`simple-crypto-lab`](https://github.com/VRAM-RAM/simple-crypto-lab).

## License 

This code is licensed under the same license as `simple-crypto-lab`, so under CECILL-B or APACHE 2.0 License, your choice.