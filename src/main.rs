use clap::{arg, command, CommandFactory, Parser};

#[derive(Parser)]
#[command(name = "transit-bd")]
#[command(
    about = "Utility command line too work with transit data of Bangladesh",
    version = "1.0"
)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Cli::parse();
    if std::env::args().len() == 1 {
        Cli::command().print_help().unwrap();
        std::process::exit(0);
    }

    if args.verbose > 0 {
        println!("Verbose mode enabled.");
    }
}
