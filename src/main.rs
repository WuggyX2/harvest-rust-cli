mod cli;
mod command;
use structopt::StructOpt;

use cli::Module;

fn main() {
    let command = Module::from_args();

    println!("{:?}", command);

    match command {
        Module::Config(opt) => {
            opt.run();
        }
        _ => {
            println!("Command has not been implemented yet");
        }
    }
}
