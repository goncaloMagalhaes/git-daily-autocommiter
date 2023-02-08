use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    let api_key = read_env_var("GITHUB_API_KEY");
    let repo_name = read_env_var("REPO_NAME");
    let username = read_env_var("USERNAME");

    let file_base_name = read_env_var("FILE_BASE_NAME");
    let date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("{}_{}", date, file_base_name);

    let contents = read_file(&file_name);

    let commit_message = if contents.is_empty() {
        format!("My daily commit - no recorded stuff for {}", date)
    } else {
        format!("My daily commit - stuff for {}", date)
    };

    // Add contents to file
    let add_file_result = Command::new("git").arg("add").arg(file_name).output();
    if !add_file_result.is_ok() {
        panic!("Failed to add file to repository");
    }

    // Commit changes
    let commit_result = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output();
    if !commit_result.is_ok() {
        panic!("Failed to commit changes");
    }

    // Push changes to GitHub
    let push_result = Command::new("git")
        .arg("push")
        .arg(format!(
            "https://{}:{}@github.com/{}/{}.git",
            username, api_key, username, repo_name
        ))
        .output();
    if !push_result.is_ok() {
        panic!("Failed to push changes to GitHub");
    }
}

fn read_env_var(var_name: &str) -> String {
    match env::var(var_name) {
        Ok(val) => val,
        Err(_) => panic!("Could not read environment variable: {}", var_name),
    }
}

fn read_file(file_name: &String) -> String {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => panic!("Could not open file: {}", file_name),
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        panic!("Could not read file: {}", file_name);
    }

    contents
}
