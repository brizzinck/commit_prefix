use crate::command::private::State;
use std::{borrow::Cow, marker::PhantomData, mem, process::Command};

mod private {
    pub trait State {}
}

pub struct Init;
pub struct Run;

impl State for Init {}
impl State for Run {}

pub struct CustomCommand<T: State> {
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
    pub fn new(command: &'static str) -> CustomCommand<Init> {
        CustomCommand {
            command,
            args: Vec::new(),
            _state: PhantomData,
        }
    }

    pub fn add_args(mut self, args: impl IntoIterator<Item = Cow<'static, str>>) -> Self {
        self.args.extend(args.into_iter().map(|v| v.to_string()));

        self
    }

    pub fn build(self) -> CustomCommand<Run> {
        self.transition()
    }
}

impl CustomCommand<Run> {
    pub fn run(&mut self) -> String {
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
