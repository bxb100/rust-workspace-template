// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::path::Path;

fn main() {
    // There are several ways to get the workspace root directory:
    // - using `CARGO_MANIFEST_DIR`
    // - using `cargo locate-project --workspace`
    // - using `CARGO_WORKSPACE_DIR` by https://doc.rust-lang.org/cargo/reference/config.html
    let workspace_root_config = Path::new(env!("CARGO_WORKSPACE_DIR")).join(".cargo");

    hawkeye_config_path(&workspace_root_config);
    taplo_config_path(&workspace_root_config);
    typos_config_path(&workspace_root_config);
    clippy_config_path(&workspace_root_config);
}

fn hawkeye_config_path(root: &Path) {
    let config_path = root.join("licenserc.toml");
    println!(
        "cargo:rustc-env=HAWKEYE_CONFIG_PATH={}",
        config_path.display()
    )
}

fn taplo_config_path(root: &Path) {
    let config_path = root.join("taplo.toml");
    println!(
        "cargo:rustc-env=TAPLO_CONFIG_PATH={}",
        config_path.display()
    )
}

fn typos_config_path(root: &Path) {
    let config_path = root.join("typos.toml");
    println!(
        "cargo:rustc-env=TYPOS_CONFIG_PATH={}",
        config_path.display()
    )
}

/// <https://doc.rust-lang.org/clippy/configuration.html>
fn clippy_config_path(root: &Path) {
    let config_path = root.join("clippy.toml");
    println!("cargo:rustc-env=CLIPPY_CONF_DIR={}", config_path.display())
}
