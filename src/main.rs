use clap::{arg, command, CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "transit-bd")]
#[command(
    about = "Utility command line too work with transit data of Bangladesh",
    version = "1.0"
)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Map match a source GeoJSON file with road network of Bangladesh")]
    MapMatch {
        #[arg(short, long)]
        source: String,
    },
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

    match args.command {
        Commands::MapMatch { source } => {
            print!("Map match sub-command invoked with source {}", source)
        }
    }
}
