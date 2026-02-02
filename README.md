this template derive from @tisonkun's[^1] workspace setup

> [!NOTE]  
> if you using hawkeye, change your license settings (`./licenserc.toml`) first.

## Lint

```shell
cargo x lint --fix
```

or 

```shell
just fix
```

## Tree

```text
.
├── .cargo
│   └── config.toml
├── .editorconfig
├── .gitattributes
├── .github
│   └── workflows
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── clippy.toml
├── core
│   ├── Cargo.toml
│   └── src
├── justfile
├── LICENSE
├── licenserc.toml
├── README.md
├── rustfmt.toml
├── taplo.toml
├── typos.toml
└── xtask
    ├── Cargo.toml
    └── src
```

- `.cargo/config.toml`: setting alias `x`
- `licenserc.toml`: https://github.com/korandoru/hawkeye#configurations
- `taplo.toml`: https://taplo.tamasfe.dev/configuration/file.html
- `typos.toml`: https://github.com/crate-ci/typos/blob/master/docs/reference.md
- `clippy.toml`: https://doc.rust-lang.org/clippy/configuration.html
- `Cargo.toml`
    - https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section
- `rust-toolchain.toml`: https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
- `rustfmt.toml`: https://github.com/rust-lang/rustfmt

## Ref

- [matklad/cargo-xtask](https://github.com/matklad/cargo-xtask)
- [rust-analyzer](https://github.com/rust-lang/rust-analyzer)

[^1]: https://github.com/apache/datasketches-rust/pull/43
