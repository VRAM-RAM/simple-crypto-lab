# `simple-ring` : Simple implementation of mathematical primitives for LWE-based crypto in Rust

`simple-ring` provides mathematical primitives, such as NTT or Polynomials, but also coefficient sampling for LWE-based schemes. For more informations and explanations, please see `/docs/simple-ring.pdf`.

## Warning 

This code isn’t audited, isn’t intended for production use, and is vulnerable to timing-based attacks.

## Quick start

To use it, simply do :
```bash
cargo add simple-ring
```
Then, in your code, you can use it as you want :
```rust
use simple_ring::*;

fn main() {
    let parameters = RingParams::new(4, 17, find_valid_omega(4, 17));
    let ntt_tables = &precalculate(&parameters);
    let first_polynomial = Polynomial::new(vec![8u64; 4]);
    let mut second_coeffs = vec![9u64; 4];
    second_coeffs[3] = 1;
    let second_polynomial = Polynomial::new(second_coeffs);
    let mul = first_polynomial.mul_ntt(&parameters, ntt_tables, &second_polynomial);
    let mul2 = first_polynomial.mul(&parameters, &second_polynomial);
    println!("The result of the mul is : {:?}", mul2);
    println!("The result of the mul ntt is : {:?}", mul);
    let ntt_first = forward_ntt(&parameters, &first_polynomial, ntt_tables);
    println!("First polynomial in NTT domain is {:?}", ntt_first);
    let sample = generate_small_sample(&parameters);
    println!("Sample is {:?}", sample);
}
```

## Tests & modifications

To run the tests, or modify the code, you can git clone the repo, and then run the tests or do any modification you want :
```bash
cargo test --release -- --nocapture
```

## Repository

`simple-ring` is a scheme from [`simple-crypto-lab`](https://github.com/VRAM-RAM/simple-crypto-lab).

## License 

This code is licensed under the same license as `simple-crypto-lab`, so under CECILL-B or APACHE 2.0 License, your choice.