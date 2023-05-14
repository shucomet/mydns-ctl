use clap::Parser;
mod data;
use data::*;

fn main() {
    let arg = CmdArg::parse();
    println!("{:#?}", arg);
}
