mod cli;
mod command;

use clap::StructOpt;
use cli::Module;

fn main() {
    let command = Module::parse();

    //println!("{:?}", command);

    match command {
        Module::Config(opt) => {
            opt.run();
        }
        _ => {
            println!("Command has not been implemented yet");
        }
    }
}
