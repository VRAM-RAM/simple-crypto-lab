# `simple-saber` : Simple implementation of Saber key exchange algorithm in Rust

The Saber scheme is a Key Encapsulation Mechanism (KEM), which allows a message to be encapsulated into a ciphertext that can only be decapsulated by the holder of the corresponding secret key. It is based on the Learning with Roundness (LWR) problem. This implementation is educational and experimental. For more informations and explanations, please see `/docs/simple-saber.pdf`. (Not created yet)

## Warning 

This code isn’t audited, isn’t intended for production use, is vulnerable to timing-based attacks, does not implement FO transform...
Also, this is **NOT** the real saber algorithm. For example, here, Light/Saber/Fire variants differ by CBD noise parameter only; the module rank is fixed.

## Quick start

To use it, simply do :
```bash
cargo add simple-saber
```
Then, in your code, you can use it as you want, e.g for encapsulating and decapsulating :
```rust
use simple_saber::{SaberDecapsulate, Saber};

fn main() {
    let saber = Saber::fire();
    let kp = saber.keygen();
    let (key, serialized_enc) = match saber.encapsulate(&kp.public_key) {
        Ok((key, serialized)) => (key, serialized),
        Err(e) => panic!("Error while encapsulating : {:?}", e),
    };
    let recovered = match serialized_enc.decapsulate(&kp) {
        Ok(key) => key,
        Err(e) => { panic!("Error while decapsulating : {:?}", e) }
    };
    assert_eq!(key, recovered, "Error : the encapsulated key and the decapsulated key doesn't match !");
    println!("Shared key is {:?} == {:?}", key, recovered);
}
```

## Parameters

The real `saber` parameters are :
<img src="https://www.researchgate.net/publication/378951552/figure/tbl2/AS:11431281853463352@1768309313394/Parameters-of-Saber-with-security-and-failure-probability-DKRV18.png" alt="Parameters of Saber with security and failure probability [DKRV18]"/></a>

In this implementation, saber scheme and parameters have been simplified.

## Tests & modifications

To run the tests, or modify the code, you can git clone the repo, and then run the tests or do any modification you want :
```bash
cargo test --release -- --nocapture
```

## Repository 

`simple-saber` is a scheme from [`simple-crypto-lab`](https://github.com/VRAM-RAM/simple-crypto-lab).

## License 

This code is licensed under the same license as `simple-crypto-lab`, so under CECILL-B or APACHE 2.0 License, your choice.