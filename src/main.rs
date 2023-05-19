use spin::Arguments;
use clap::Parser;

fn main() {
    let arguments : Arguments = Arguments::parse();
    spin::run(&arguments);
}
