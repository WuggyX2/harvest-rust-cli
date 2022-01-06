use crate::command::config::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = "harvest",
    about = "A CLI for the Harvest API",
    author = "Matti Virkkunen"
)]
pub enum Module {
    Config(Config),
    Sync,
    Log,
}
/*
#[derive(Debug, StructOpt)]
struct Log {
    #[structopt(subcommand)]
    method: LogMethod,
}

#[derive(Debug, StructOpt)]
enum LogMethod {
    //hello
    Start,
    Update,
    Delete,
} */
