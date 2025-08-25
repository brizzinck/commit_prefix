use clap::Parser;
use clap_derive::Parser;
use std::process::Command;

#[derive(Debug)]
pub struct Commit {
    hash: String,
    name: String,
}

impl Commit {
    pub fn new(hash: String, name: String) -> Self {
        Self { hash, name }
    }
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    ancestry_path: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let cmd = String::from_utf8(
        Command::new("git")
            .arg("log")
            .arg("--pretty=format:\"%h,%s\"")
            .output()
            .expect("failed to output")
            .stdout,
    )
    .expect("failed to parse utf8");

    let commits = cmd
        .lines()
        .map(|v| {
            let lol = v.split(',').map(|v| v.to_owned()).collect::<Vec<String>>();

            debug_assert!(lol.len() == 2);

            Commit::new(lol[0].clone(), lol[1].clone())
        })
        .collect::<Vec<Commit>>();

    println!("{commits:?}");

    for commit in commits {}
}
