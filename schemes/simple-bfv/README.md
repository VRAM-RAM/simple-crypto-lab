# `simple-bfv` : Simple implementation of BFV FHE scheme in Rust

The BFV (Brakerski/Fan-Vercauteren) is a Fully Homomorphic Encryption  scheme  (FHE),  which  allows data  to  be  processed  while  still encrypted. This implementation is educational and experimental. For more informations and explanations, please see `/docs/simple-bfv.pdf`.

## Warning 

This code isn’t audited, isn’t intended for production use, is vulnerable to timing-based attacks...
So please don’t use it seriously. But, you can use it to experiment with BFV encryption, implement an e-voting or PIR with it, for example.

## Quick start

To use it, simply do :
```bash
cargo add simple-bfv
```
Then, in your code, you can use it as you want :
```rust
use simple_bfv::{BFV, BFVPlaintext};

fn main() {
    let bfv = BFV::for_test();
    let a = bfv.generate_public_a();
    let s = bfv.generate_secret_key();
    let b = bfv.generate_public_b(&a, &s);
    let plaintext = BFVPlaintext::new("Bonjour, voici bfv", &bfv);
    let ciphertext = bfv.encrypt(&plaintext, &a, &b);
    println!("ciphertext is {:?}", ciphertext);
    let decrypted = bfv.decrypt(&ciphertext, &s);
    println!("Decrypted is {}", decrypted);
}
```

## Tests & modifications

To run the tests, or modify the code, you can git clone the repo, and then run the tests or do any modification you want :
```bash
cargo test --release -- --nocapture
```


## Repository

`simple-bfv` is a scheme from [`simple-crypto-lab`](https://github.com/VRAM-RAM/simple-crypto-lab).

## License 

This code is licensed under the same license as `simple-crypto-lab`, so under CECILL-B or APACHE 2.0 License, your choice.