// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
use argus::Command;
use log::error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct CLI {
    #[structopt(
        short = "q",
        long = "quiet",
        help = "Quiet mode",
        conflicts_with = "verbose"
    )]
    quiet: bool,

    #[structopt(
        short = "v",
        long = "verbose",
        parse(from_occurrences),
        conflicts_with = "quiet",
        help = "Verbose mode"
    )]
    verbose: usize,

    #[structopt(help = "command")]
    command: String,
}

fn main() {
    let args = CLI::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbose)
        .init()
        .unwrap();

    match Command::from_str(&args.command) {
        Err(err) => error!("{}", err),
        Ok(_) => {}
    }
}

// vim: set tabstop=4 sw=4 expandtab:
