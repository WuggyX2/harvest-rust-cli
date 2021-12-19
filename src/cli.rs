use crate::command::auth::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "harvest", about = "Kikkelis kokkelis")]
pub enum Module {
    Auth(Auth),
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
