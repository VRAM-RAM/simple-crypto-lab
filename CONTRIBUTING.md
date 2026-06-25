<!--
SPDX-FileCopyrightText: 2026 Olruix and simple-crypto-lab contributors
SPDX-License-Identifier: CECILL-B or Apache-2.0
-->

# Contributing to `simple-crypto-lab`

We welcome contributions from everyone. Whether you are fixing a bug, improving
documentation, proposing a new scheme... , your help is appreciated.

## Table of contents

- [Reporting issues](#reporting-issues)
  - [Feature Requests](#feature-requests)
  - [Submitting a bug report](#submitting-a-bug-report)
- [Development Environment Setup](#development-environment-setup)
  - [Install Rust](#install-rust)
  - [Clone the Repository](#clone-the-repository)
- [Adding tests](#adding-tests)
  - [For new features](#for-new-features)
  - [For bug fixes](#for-bug-fixes)
- [Verifying Your Changes](#verifying-your-changes)
  - [Run Tests](#run-tests)
- [Dependency Policy](#dependency-policy)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [License](#license)


We are committed to providing a friendly, safe, and welcoming environment for all.

## Reporting issues

### Feature Requests

If you have an idea for a new feature, please open an Issue with a `[Feature]` header.

> [!NOTE]
> `simple-crypto-lab` needs to remain educative and *simple*. Features that add a lot of complexity and low-level adaptation as SIMD will be rejected.

### Submitting a bug report


If you find a bug, please create an issue with a `[BUG]` header.
Provide as much detail as possible, including steps to reproduce the issue and
the expected behavior.

## Development Environment Setup

### Install Rust

To develop on `simple-crypto-lab`, you will need [Rust](https://rust-lang.org/tools/install) installed.

The `nightly` toolchain is recommended but not obligatory.

### Clone the Repository

```bash
git clone https://github.com/VRAM-RAM/simple-crypto-lab
cd simple-crypto-lab
```

## Adding tests

### For new features

If your change introduces a new feature, please ensure it is appropriately tested:

- Functions can typically be tested using **unit testing** within the source
  file (see [documentation](https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests))

### For bug fixes

Bug fixes should almost always be accompanied with a [regression
test](https://en.wikipedia.org/wiki/Regression_testing). This can be a unit test
(or, more rarely, an integration test) that should pass with your new changes
but would have failed beforehand. This ensures similar faults cannot re-emerge
without being detected in the future.

## Verifying Your Changes

Before submitting a Pull Request, please run the following checks.

### Run Tests

Ensure all unit tests pass.

```bash
cargo test -- --nocapture
```

## Dependency Policy

Kibi aims to have minimal dependencies.

- **Production Dependencies:** Do **not** add new dependencies to `simple-crypto-lab` without
explicitly discussing it in an issue first. Most features should be implemented
using the Rust standard library.
- **Dev Dependencies:** Adding dependencies for testing or development tools
(e.g. within `[dev-dependencies]`, or [`xtask/Cargo.toml`](xtask/Cargo.toml))
is generally acceptable, provided they are justified.

## Submitting a Pull Request

Changes to code for Kibi are made through Pull Requests on GitHub.

> [!TIP]
> If you are new to contributing to an open source project on GitHub, you can
> refer to the [_Creating a pull request_](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request)
> article in the GitHub documentation.

1. **Branching:** Create a new branch for your work. We don't enforce a naming
   convention, but descriptive names are helpful.
2. **Commits:** Similarly, we do not enforce a convention for commit messages,
   but please ensure they are descriptive enough.
3. **Push & Open PR:** Push your branch to GitHub and open a Pull Request.
4. **Description:** Fill out the PR description clearly. 
5. **Checks:** After you submit the pull request, continuous integration checks
   will be run using GitHub Actions to enforce that the commit conforms to `simple-crypto-lab`'s
   quality guidelines (tests, simplicity, etc.). Please ensure all checks pass,
   fixing surfaced issues as needed.

## License

Any contribution submitted for inclusion in Kibi by you shall be dual licensed under:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [CECILL-B License](LICENSE-CECILL-B-EN)

without any additional terms or conditions.