/*
 * Copyright 2023 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::env;

fn main() {
    let rustc = rustc_version::version_meta().unwrap();
    println!(
        "cargo:rustc-env=G3_BUILD_RUSTC_VERSION={}",
        rustc.short_version_string
    );
    println!("cargo:rustc-env=G3_BUILD_RUSTC_CHANNEL={:?}", rustc.channel);

    println!(
        "cargo:rustc-env=G3_BUILD_HOST={}",
        env::var("HOST").unwrap()
    );
    println!(
        "cargo:rustc-env=G3_BUILD_TARGET={}",
        env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-env=G3_BUILD_PROFILE={}",
        env::var("PROFILE").unwrap()
    );
    println!(
        "cargo:rustc-env=G3_BUILD_OPT_LEVEL={}",
        env::var("OPT_LEVEL").unwrap()
    );
    println!(
        "cargo:rustc-env=G3_BUILD_DEBUG={}",
        env::var("DEBUG").unwrap()
    );

    if let Ok(v) = env::var("G3_PACKAGE_VERSION") {
        println!("cargo:rustc-env=G3_PACKAGE_VERSION={v}");
    }
}
