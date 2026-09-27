# Tests

This directory contains integration tests for this repo. Tests may be run using loom, shuttle, miri, or default.

## Running

To run tests:

```bash
  cargo test --locked --all-features
```

To run tests using `miri`:

```bash
  cargo miri test --locked --all-features
```

To run `loom` tests:

```bash
  LOOM_MAX_PREEMPTIONS=2 RUSTFLAGS="--cfg loom" cargo test --release --lib
```

To run `shuttle` tests:

```bash
  RUSTFLAGS="--cfg shuttle" cargo test --release --lib
```
