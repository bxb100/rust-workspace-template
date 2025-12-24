this template derive from @tisonkun's[^1] workspace setup

```shell
cargo x lint --fix
```

tree:

```text
.
├── .cargo
│   ├── config.toml
│   ├── licenserc.toml
│   ├── taplo.toml
│   └── typos.toml
├── .editorconfig
├── .gitattributes
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── rust-toolchain.toml
├── rustfmt.toml
└── xtask
    ├── build.rs
    ├── Cargo.toml
    └── src
```

- `.cargo/config.toml`: setting alias `x`
- `.cargo/licenserc.toml`: https://github.com/korandoru/hawkeye#configurations
- `.cargo/taplo.toml`: https://taplo.tamasfe.dev/configuration/file.html
- `.cargo/typos.toml`: https://github.com/crate-ci/typos/blob/master/docs/reference.md
- `Cargo.toml`
  - https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section
- `rust-toolchain.toml`: https://rust-lang.github.io/rustup/overrides.html#the-toolchain-file
- `rustfmt.toml`: https://github.com/rust-lang/rustfmt

[^1]: https://github.com/apache/datasketches-rust/pull/43
