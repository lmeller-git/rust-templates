// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_{{crate_name | snake_case}}_extendr(void *dll);

void R_init_{{crate_name | snake_case}}(void *dll) {
    R_init_{{crate_name | snake_case}}_extendr(dll);
}
