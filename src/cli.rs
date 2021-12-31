use crate::command::auth::*;
use crate::command::config::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "harvest", about = "Kikkelis kokkelis")]
pub enum Module {
    Config(Config),
    Sync,
    Log,
}

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
}
