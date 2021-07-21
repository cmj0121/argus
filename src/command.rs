// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use crate::error::Error;
use pest::Parser as ParserTrait;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Parser;

pub struct Command {}

impl Command {
    pub fn from_str(s: &str) -> Result<(), Error> {
        match Parser::parse(Rule::command, s) {
            Err(err) => Err(Error::Message(format!("command: {}", err))),
            Ok(pairs) => Ok(()),
        }
    }
}

// vim: set tabstop=4 sw=4 expandtab:
