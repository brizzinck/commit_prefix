use crate::command::CustomCommand;
use clap::Parser;
use clap_derive::Parser;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Commit {
    hash: String,
    name: String,
}

impl Commit {
    pub fn new(hash: String, name: String) -> Self {
        Self { hash, name }
    }

    pub fn amend_message() {}
}

#[derive(Parser, Debug)]
pub struct GitCli {
    #[arg(short, long)]
    ancestry_path: Option<String>,
    #[arg(short, long)]
    since_symbol: Option<char>,
    #[arg(short, long)]
    until_symbol: Option<char>,
    #[arg(short, long)]
    prefix: String,
}

pub type GitArgs = Vec<Cow<'static, str>>;

impl GitCli {
    pub fn run() {
        let mut cli = GitCli::parse();

        let args = cli.parse_args();

        let commits = cli.parse_commits(args);

        println!("{cli:?}");

        cli.add_prefix(commits);
    }

    pub fn parse_args(&mut self) -> GitArgs {
        let mut args = vec!["log", "--pretty=format:\"%h,%s\""]
            .into_iter()
            .map(Into::into)
            .collect::<GitArgs>();

        if let Some(path) = self.ancestry_path.clone() {
            args.push(path.into());
        }

        args
    }

    pub fn parse_commits(&self, args: GitArgs) -> Vec<Commit> {
        const MAIN_GIT_CMD: &str = "git";

        CustomCommand::new(MAIN_GIT_CMD)
            .add_args(args)
            .build()
            .run()
            .lines()
            .map(|v| {
                let lol = v.split(',').map(|v| v.to_owned()).collect::<Vec<String>>();

                debug_assert!(lol.len() == 2);

                Commit::new(lol[0].clone(), lol[1].clone())
            })
            .collect::<Vec<Commit>>()
    }

    pub fn add_prefix(&self, commits: Vec<Commit>) {
        for mut commit in commits {
            println!("{commit:?}");

            let mut since_index = 0;
            let mut until_index = commit.name.len() - 1;

            if let Some(symbol) = self.since_symbol {
                since_index = commit.name.find(symbol).unwrap_or(0);
            }

            if let Some(symbol) = self.until_symbol {
                until_index = commit.name.find(symbol).unwrap_or(0);
            }

            commit
                .name
                .replace_range(since_index + 1..until_index, &self.prefix);

            println!("{commit:?}");
        }
    }
}
