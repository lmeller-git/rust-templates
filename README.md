# rust-template-no_std-bin

This repository contains an opinionated template for no_std rust bins. Some workflows may have to be adjusted for your usecase.

Test using rusts standard harness:

```bash
cargo test
```

Build for your target:

```bash
carog build --target <target-triple>
```


## Usage

This repo may be used as a template via cargo-generate.

```bash
cargo generate lmeller-git/rust-templates --branch bin-no_std --name <your-crate-name>
```
