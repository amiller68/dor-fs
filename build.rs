// build.rs
use std::fs::File;
use std::io::Write;

use dotenv::from_filename;

const WEB_CONFIG_PATH: &str = "web.config";
const WEB_DEV_CONFIG_PATH: &str = "web.config.dev";
const WEB_ENV_RS_PATH: &str = "./src/wasm/env.rs";

/// Initialize our Web Build based on the on-disk config
/// We do this because Trunk does not bundle Env variables at build or runtime
fn web() {
    // Check if we're building for production or development
    let profile = std::env::var("PROFILE").unwrap();
    let config_path = match profile.as_str() {
        "release" => WEB_CONFIG_PATH,
        "debug" => WEB_DEV_CONFIG_PATH,
        _ => panic!("Unknown profile: {}", profile),
    };
    println!("cargo:rerun-if-changed={}", config_path);

    let mut f = File::create(WEB_ENV_RS_PATH).unwrap();
    let version = env!("CARGO_PKG_VERSION");
    from_filename(config_path).ok();

    // Create our env.rs file
    f.write_all(b"// This file is automatically generated by build.rs\n\n")
        .unwrap();

    f.write_all(b"#![allow(dead_code)]\n\n").unwrap();

    let key = "APP_VERSION";
    let line = format!(
        "pub const {}: &str = \"{}\";\n",
        key,
        version.replace('\"', "\\\"")
    );
    f.write_all(line.as_bytes()).unwrap();

    for (key, value) in std::env::vars() {
        if key.starts_with("APP_") {
            let line = format!(
                "pub const {}: &str = \"{}\";\n",
                key,
                value.replace('\"', "\\\"")
            );
            f.write_all(line.as_bytes()).unwrap();
        }
    }
}

fn report_build_profile() {
    println!(
        "cargo:rustc-env=BUILD_PROFILE={}",
        std::env::var("PROFILE").unwrap()
    );
}

fn report_enabled_features() {
    let mut enabled_features: Vec<&str> = Vec::new();

    if enabled_features.is_empty() {
        enabled_features.push("none");
    }

    println!(
        "cargo:rustc-env=BUILD_FEATURES={}",
        enabled_features.join(",")
    );
}

fn report_repository_version() {
    let version = match std::env::var("CI_BUILD_REF") {
        Ok(val) if !val.is_empty() => val,
        _ => {
            let git_describe = std::process::Command::new("git")
                .args(["describe", "--always", "--dirty", "--long", "--tags"])
                .output()
                .unwrap();

            String::from_utf8(git_describe.stdout).unwrap()
        }
    };

    println!("cargo:rustc-env=REPO_VERSION={}", version);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_arch == "wasm32" {
        println!("cargo:rustc-env=OUT_DIR=krondor-org-web");
        web();
    }
    report_build_profile();
    report_enabled_features();
    report_repository_version();
}
