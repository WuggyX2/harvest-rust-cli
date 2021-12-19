mod cli;
mod command;
use structopt::StructOpt;

fn main() {
    let command = cli::Module::from_args();

    println!("{:?}", command);
}
