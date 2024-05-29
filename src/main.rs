use mwc;
use clap::Parser;



fn main() {
    let cli = mwc::Cli::parse();

    mwc::run(cli);
}
