mod cli;

use cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

}
