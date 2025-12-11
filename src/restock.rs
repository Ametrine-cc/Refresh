use std::fs;
use std::process::Command;

pub fn mk_project(path: &str, lang: &str) {
    match lang {
        "Rust" => {
            println!("Creating Rust project...");
            let mut cmd = Command::new("cargo");
            cmd.arg("init");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("Rust project created successfully!");
        }

        "Python" => {
            println!("Creating Python project...");
            let mut cmd = Command::new("python");
            cmd.arg("-m");
            cmd.arg("venv");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("Python project created successfully!");
        }

        "JavaScript" => {
            println!("Creating JavaScript project...");
            let mut cmd = Command::new("npm");
            cmd.arg("init");
            cmd.arg("-y");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("JavaScript project created successfully!");
        }

        "Go" => {
            println!("Creating Go project...");
            let mut cmd = Command::new("go");
            cmd.arg("mod");
            cmd.arg("init");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("Go project created successfully!");
        }

        "Java" => {
            println!("Creating Java project...");
            let mut cmd = Command::new("java");
            cmd.arg("-jar");
            cmd.arg("javac");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("Java project created successfully!");
        }

        "C#" => {
            println!("Creating C# project...");
            let mut cmd = Command::new("dotnet");
            cmd.arg("new");
            cmd.arg("console");
            cmd.arg("--output");
            cmd.arg(path);
            cmd.status().expect("failed to create project");
            println!("C# project created successfully!");
        }

        &_ => todo!(),
    }
}
