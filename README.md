# simple-crypto-lab
\
![simple-crypto-lab](https://img.shields.io/badge/simple_crypto_lab-rust-orange)
![Version](https://img.shields.io/github/v/release/VRAM-RAM/simple-crypto-lab)
![License](https://img.shields.io/github/license/VRAM-RAM/simple-crypto-lab)
\
This workspace contains educational implementations of cryptographic schemes based on 
ring learning with errors (RLWE). The code is designed for **learning and experimentation**, 
not for production use.

## Warning

**This lab is for educational and experimental purposes only.**

It is **not** :
+ Audited for security
+ Constant-time
+ Suitable for protecting sensitive datas

For production use, consider audited library, like : 
+ [Microsoft SEAL](https://github.com/microsoft/SEAL)
+ [OpenFHE](https://openfhe.org/)
+ [liboqs](https://github.com/open-quantum-safe/liboqs)

If you need a Rust implementation, please see [fhe.rs](https://github.com/tlepoint/fhe.rs) or [RustCrypto](https://github.com/rustcrypto),  but it isn't audited.

## Workspace structure

```bash
.
├── Cargo.lock
├── Cargo.toml
├── docs 
│   ├── pdf
│   │   ├── simple-bfv.pdf
│   │   └── simple-ring.pdf 
│   └── typst
│       ├── simple-bfv.typ
│       └── simple-ring.typ
├── schemes
│   ├── simple-bfv #simple and educative implementation of BFV
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── config.rs
│   │       ├── find_parameters.rs
│   │       ├── lib.rs
│   │       ├── plaintext.rs
│   │       └── scheme.rs
│   └── simple-saber #simple and educative implementation of SABER KEM
│       ├── Cargo.toml
│       └── src
│           ├── config.rs
│           ├── encapsulation.rs
│           ├── error.rs
│           ├── exportable_params.rs
│           ├── lib.rs
│           ├── scheme.rs
│           └── types.rs
└── simple-ring #Primitives' crate
    ├── Cargo.toml
    └── src
        ├── bitwriting.rs
        ├── encoding.rs
        ├── lib.rs
        ├── modular.rs
        ├── ntt.rs
        ├── polys.rs
        ├── ring.rs
        └── sampling.rs
```
> Saber doc is not yet created but will be !


## Quick Start

### Prerequisites 

- Rust 1.70+ 
- Optional : Tyspt, if you want to edit / compile the documentation

### Build the workspace

```bash
# Build all crates :

cargo build --workspace -release

# Run all tests :

cargo test --workspace --release

# Run with parallel NTT & parallel polynomial code (requires rayon, and it's experimental) :

cargo build --workspace --release --features parallel

# Run tests with output :

cargo test -- --nocapture
```


### Documentation

You can find the documentation either on `docs.rs`, but it's very minimalist, or in `/docs/`, where you can find the full documentation for each crate.

#### Build Documentation 

If you need to build it :

```bash
# Rust API documentation :

cargo doc --workspace --open

# Crates full documentation : 

cd docs/typst
typst compile simple-ring.typ
typst compile simple-bfv.typ
```

### What you'll learn 

+ The Ring : how it works and why it's used
+ NTT : Why Number Theoretic Transform is so efficient and how does it work
+ BFV Scheme : Key generation, encryption, decryption, and homomorphic operations
+ Noise Management : How noise grows and why it limits computation depth
+ Coefficients sampling
+ LWR & Saber scheme. Not the real one ( real saber doesn't use NTT, this one yes for simplicity and compatibility with `simple-ring` )

### Features 

`parallel` : Enable multithreaded NTT and polynomial code with rayon. In fact, it can be less efficient than the single thread code...

### Contributing

Contributions are welcome, especially : 

+ Additional examples or tests
+ Bug reports
+ Corrections

### License 

The code is licensed under CECILL-B or Apache-2.0, your choice.


### Contact

Author & Developer : Olruix ([VRAM-RAM](https://github.com/VRAM-RAM))

