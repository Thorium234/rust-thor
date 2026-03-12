(base) thorium@thorium-HP-ProBook-x360-11-G2-EE:~/Desktop/programming/2026/Rust/milestone1/example5$ cargo init
    Creating binary (application) package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
(base) thorium@thorium-HP-ProBook-x360-11-G2-EE:~/Desktop/programming/2026/Rust/milestone1/example5$ vi src/main.rs 
(base) thorium@thorium-HP-ProBook-x360-11-G2-EE:~/Desktop/programming/2026/Rust/milestone1/example5$ cargo run
   Compiling example5 v0.1.0 (/home/thorium/Desktop/programming/2026/Rust/milestone1/example5)
error[E0432]: unresolved import `rand`
 --> src/main.rs:3:5
  |
3 | use rand::Rng;
  |     ^^^^ use of unresolved module or unlinked crate `rand`
  |
  = help: if you wanted to use a crate named `rand`, use `cargo add rand` to add it to your `Cargo.toml`

error[E0433]: failed to resolve: use of unresolved module or unlinked crate `rand`
 --> src/main.rs:6:21
  |
6 | let secret_number = rand::thread_rng().gen_range(1..=100);
  |                     ^^^^ use of unresolved module or unlinked crate `rand`
  |
  = help: if you wanted to use a crate named `rand`, use `cargo add rand` to add it to your `Cargo.toml`

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
error: could not compile `example5` (bin "example5") due to 2 previous errors
(base) thorium@thorium-HP-ProBook-x360-11-G2-EE:~/Desktop/programming/2026/Rust/milestone1/example5$ cargo add rand
    Updating crates.io index
      Adding rand v0.10.0 to dependencies
             Features:
             + alloc
             + std
             + std_rng
             + sys_rng
             + thread_rng
             - chacha
             - log
             - serde
             - simd_support
             - unbiased
    Updating crates.io index
     Locking 44 packages to latest Rust 1.93.1 compatible versions
      Adding anyhow v1.0.102
      Adding bitflags v2.11.0
      Adding cfg-if v1.0.4
      Adding chacha20 v0.10.0
      Adding cpufeatures v0.3.0
      Adding equivalent v1.0.2
      Adding foldhash v0.1.5
      Adding getrandom v0.4.2
      Adding hashbrown v0.15.5
      Adding hashbrown v0.16.1
      Adding heck v0.5.0
      Adding id-arena v2.3.0
      Adding indexmap v2.13.0
      Adding itoa v1.0.17
      Adding leb128fmt v0.1.0
      Adding libc v0.2.183
      Adding log v0.4.29
      Adding memchr v2.8.0
      Adding prettyplease v0.2.37
      Adding proc-macro2 v1.0.106
      Adding quote v1.0.45
      Adding r-efi v6.0.0
      Adding rand v0.10.0
      Adding rand_core v0.10.0
      Adding semver v1.0.27
      Adding serde v1.0.228
      Adding serde_core v1.0.228
      Adding serde_derive v1.0.228
      Adding serde_json v1.0.149
      Adding syn v2.0.117
      Adding unicode-ident v1.0.24
      Adding unicode-xid v0.2.6
      Adding wasip2 v1.0.2+wasi-0.2.9
      Adding wasip3 v0.4.0+wasi-0.3.0-rc-2026-01-06
      Adding wasm-encoder v0.244.0
      Adding wasm-metadata v0.244.0
      Adding wasmparser v0.244.0
      Adding wit-bindgen v0.51.0
      Adding wit-bindgen-core v0.51.0
      Adding wit-bindgen-rust v0.51.0
      Adding wit-bindgen-rust-macro v0.51.0
      Adding wit-component v0.244.0
      Adding wit-parser v0.244.0
      Adding zmij v1.0.21
(base) thorium@thorium-HP-ProBook-x360-11-G2-EE:~/Desktop/programming/2026/Rust/milestone1/example5$ 

