use clap::Parser;
use clap_derive::Parser;
use std::{borrow::Cow, marker::PhantomData, mem, process::Command};

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

trait State {}

struct Init;
struct Run;

impl State for Init {}
impl State for Run {}

struct CustomCommand<T: State> {
    command: &'static str,
    args: Vec<String>,
    _state: PhantomData<T>,
}

impl<T: State> CustomCommand<T> {
    fn transition<S: State>(self) -> CustomCommand<S> {
        CustomCommand {
            command: self.command,
            args: self.args,
            _state: PhantomData,
        }
    }
}

impl CustomCommand<Init> {
    fn new(command: &'static str) -> CustomCommand<Init> {
        CustomCommand {
            command,
            args: Vec::new(),
            _state: PhantomData,
        }
    }

    fn add_args(mut self, args: impl IntoIterator<Item = Cow<'static, str>>) -> Self {
        self.args.extend(args.into_iter().map(|v| v.to_string()));

        self
    }

    fn build(self) -> CustomCommand<Run> {
        self.transition()
    }
}

impl CustomCommand<Run> {
    fn run(&mut self) -> String {
        String::from_utf8(
            Command::new(self.command)
                .args(mem::take(&mut self.args))
                .output()
                .expect("failed to output")
                .stdout,
        )
        .expect("failed to parse utf8")
    }
}

fn main() {
    let cli = Cli::parse();

    let mut args = vec!["log", "--pretty=format:\"%h,%s\""]
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Cow<'static, str>>>();

    if let Some(path) = cli.ancestry_path {
        println!("{path}");
        args.push(path.into());
    }

    let cmd = CustomCommand::new("git").add_args(args).build().run();

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
