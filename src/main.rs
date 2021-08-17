// Copyright 2021 cmj <cmj@cmj.tw>. All right reserved.
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
}

fn main() {
    let args = CLI::from_args();

    stderrlog::new()
        .module(module_path!())
        .quiet(args.quiet)
        .verbosity(args.verbose)
        .init()
        .unwrap();
}

// vim: set tabstop=4 sw=4 expandtab:
