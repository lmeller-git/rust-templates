# rust-templates

This repo contains various templates for rust projects. Templates are organized in branches.

## Templates

| Branch | Description |
|--------|-------------|
| `lib-concurrency-no_std` | Concurrency-focused library template with `no_std` support, loom/shuttle/echeneis testing, benchmarks, and profiling |
| `lib-no_std` | General purpose `no_std` library template with sync primitives and loom/shuttle testing |
| `bin-no_std` | `no_std` binary template with custom panic handler and entry point |
| `pyo3` | Python bindings template using PyO3 with workspace structure |
| `rextendr` | R package template using extendr with full R build integration |

## Usage

All templates use `cargo-generate` with templated variables. Required variables must be provided; optional ones have defaults.

### Quick start

```bash
cargo generate lmeller-git/rust-templates --branch <branch> --name <crate-name>
```

### Template variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `crate_name` | yes | - | Name of the generated crate (kebab-case) |
| `authors` | yes | - | Author(s) in Cargo format: `["Name <email>"]` |
| `description` | yes* | - | Crate description (*required for lib templates) |
| `license` | no | `MIT` | SPDX license identifier |
| `repository` | no | - | GitHub/GitLab repository URL |
| `documentation` | no | - | Documentation URL (e.g., docs.rs) |
| `rust_version` | no | `1.85.0` | Minimum supported Rust version (MSRV) |
| `year` | no | current year | Copyright year |

### Examples

**Basic library:**
```bash
cargo generate lmeller-git/rust-templates --branch lib-no_std \
  --name my-lib \
  --define authors="Jane Doe <jane@example.com>" \
  --define description="A cool no_std library"
```

**Concurrency library with custom license:**
```bash
cargo generate lmeller-git/rust-templates --branch lib-concurrency-no_std \
  --name my-concurrent-lib \
  --define authors="Jane Doe <jane@example.com>" \
  --define description="Lock-free data structures" \
  --define license="Apache-2.0"
```

**Python bindings:**
```bash
cargo generate lmeller-git/rust-templates --branch pyo3 \
  --name my_py_module \
  --define authors="Jane Doe <jane@example.com>"
```

**R package:**
```bash
cargo generate lmeller-git/rust-templates --branch rextendr \
  --name my_r_pkg \
  --define authors="Jane Doe <jane@example.com>"
```

### Using with `--define` for multiple variables

```bash
cargo generate lmeller-git/rust-templates --branch lib-concurrency-no_std \
  --name my-crate \
  --define authors="Jane Doe <jane@example.com>" \
  --define description="My awesome crate" \
  --define repository="https://github.com/myuser/my-crate" \
  --define documentation="https://docs.rs/my-crate" \
  --define rust_version="1.80.0"
```

### Non-interactive mode (CI/CD)

```bash
cargo generate lmeller-git/rust-templates --branch lib-no_std \
  --name my-lib \
  --define authors="CI Bot <ci@example.com>" \
  --define description="Generated library" \
  --force
```
