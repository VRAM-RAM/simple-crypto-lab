# simple-crypto-lab

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

If you need a Rust implementation, please see [fhe.rs](https://github.com/tlepoint/fhe.rs), but it isn't audited.

## Workspace structure

```bash
.
├── Cargo.toml 
├── docs #Documentations for the crates (pdf & typst)
│   ├── simple-bfv.typ 
│   └── simple-ring.typ
├── README.md
├── schemes
│   └── simple-bfv #implemented schemes (only BFV for now)
│       ├── Cargo.toml
│       └── src
│           ├── config.rs
│           ├── find_parameters.rs
│           ├── lib.rs
│           ├── plaintext.rs
│           └── scheme.rs
└── simple-ring # simple-ring, the crate that contains mathematical primitives for all the workspace members
    ├── Cargo.toml 
    └── src
        ├── errors.rs
        ├── lib.rs
        ├── modular.rs
        ├── ntt.rs
        ├── polys.rs
        ├── ring.rs
        └── sampling.rs
```

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
```


### Documentation

You can find the documentation either on `docs.rs`, but it's very minimalist, or in `/docs/`, where you can find the full documentation for each crate.

#### Build Documentation 

If you need to build it :

```bash
# Rust API documentation :

cargo doc --workspace --open

# Crates full documentation : 

cd docs
typst compile simple-ring.typ
typst compile simple-bfv.typ*
```

### What you'll learn 

+ The Ring : how it works and why it's used
+ NTT : Why Number Theoretic Transform is so efficient and how does it work
+ BFV Scheme : Key generation, encryption, decryption, and homomorphic operations
+ Noise Management : How noise grows and why it limits computation depth
+ Coefficients sampling

### Features 

`parallel` : Enable multithreaded NTT and polynomial code with rayon. In fact, it can be less efficient than the single thread code...

### Performances 

### Contributing

Contributions are welcome, especially : 

+ Additional examples or tests
+ Bug reports
+ Corrections

### License 

The code is licensed under MIT License. Please see `/license/` for more informations.


### Contact

Author & Developer : Olruix ([VRAM-RAM](https://github.com/VRAM-RAM))

