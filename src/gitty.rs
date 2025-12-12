// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright (c) 2025 Ametrine Foundation <business@ametrine.cc>

use std;
use std::process::Command;

pub fn init(path: &str) {
    let mut cmd = Command::new("git");
    cmd.arg("init");
    cmd.arg(path);
    cmd.status().expect("failed to initialize git repository");
    println!("Git repository initialized successfully!");
}
