# Rust for AI

**Learn AI and machine learning by building small examples in Rust.** Open source, beginner-friendly, and focused on working code.

> **Early development:** the first working example is simple linear regression. More algorithms and tooling may be added over time.

## Quick start

Install [Rust and Cargo](https://rustup.rs/), then clone and run:

```sh
git clone https://github.com/edujbarrios/rust-for-ai.git
cd rust-for-ai
cargo run
```

Expected output:

```text
Rust for AI — linear regression
Slope: 2
Intercept: 1
Prediction for x=6: 13
```

The example trains a straight line `y = 2x + 1` on five data points, then predicts the value at `x = 6`. It runs locally without downloading models, data, or third-party crates.

## Use the library

The `LinearRegression` struct exposes `fit` and `predict`:

```rust
use rust_for_ai::LinearRegression;

let x = [0.0, 1.0, 2.0, 3.0];
let y = [1.0, 3.0, 5.0, 7.0];

let model = LinearRegression::fit(&x, &y)?;
println!("{}", model.predict(6.0)); // 13
# Ok::<(), Box<dyn std::error::Error>>(())
```

Read [`src/lib.rs`](src/lib.rs) for the fit implementation and error handling, or [`src/main.rs`](src/main.rs) for the executable example. This example is educational rather than a production-grade ML implementation.

## Commands

| Command | Purpose |
| --- | --- |
| `cargo run` | Run the linear regression example |
| `cargo test` | Run unit tests |
| `cargo fmt --all -- --check` | Verify Rust formatting |
| `cargo clippy --all-targets -- -D warnings` | Check code quality |
| `cargo build --release` | Build an optimized executable |

GitHub Actions runs formatting, linting, and tests on pull requests and pushes to `main`.

## Contribute

Contributions, bug reports, ideas, and documentation improvements are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) and [open issues](https://github.com/edujbarrios/rust-for-ai/issues).

## License

Distributed under the custom **BSD 3-Clause Attribution License** in [LICENSE](LICENSE). It adds a **fourth, mandatory acknowledgment condition** and is not the standard SPDX `BSD-3-Clause` text. Read the full license before reuse or redistribution.
