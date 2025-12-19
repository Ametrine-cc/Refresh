// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright (c) 2025 Ametrine Foundation <business@ametrine.cc>
//
use colorama::Colored;
use std::env;
use std::process::Command;
mod configuration;
mod gitty;
mod restock;

fn main() {
    let mut cmd = Command::new("clear");
    cmd.status().expect("failed to clear screen");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "init" => {
            if args.len() < 3 {
                let path = ".";
                configuration::choose_lang(path);
                return;
            }
            let path = &args[2];
            configuration::choose_lang(path);
            println!("Refresh completed successfully!");
        }
        "--help" | "-h" => {
            print_usage();
        }
        _ => {
            let mut s = String::from("Error: Unknown command");
            let mut command = String::from(command);
            command.color("green").style("bold");
            s.color("red").style("bold");
            eprintln!("{} '{}'", s, command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage: refresh <command> [arguments]");
    println!("\nCommands:");
    println!("  init <path>    Initialize a new project at the given path");
    println!("  --help, -h     Show this help message");
}
