use clap::{Parser, Subcommand};
use todo::storage;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(aliases = &["a"])]
    Add {
        entry: Vec<String>,
    },
    #[clap(aliases = &["u"])]
    Update {
        index: i32,
        entry: Vec<String>,
    },
    #[clap(aliases = &["r", "rm"])]
    Remove {
        index: i32,
    },
    #[clap(aliases = &["done", "c"])]
    Check {
        index: i32,
    },
    Uncheck {
        index: i32,
    },
    #[clap(aliases = &["l", "ls"])]
    List {
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let store = storage::TaskStorage::new();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Add { entry } => {
            store.add(&entry.join(" "));
        }
        Commands::Update { index, entry } => {
            store.update(*index, &entry.join(" "));
        }
        Commands::Remove { index } => {
            store.remove(*index);
        }
        Commands::Check { index } => {
            store.check(*index);
        }
        Commands::Uncheck { index } => {
            store.uncheck(*index);
        }
        Commands::List { all } => {
            println!("{}", store.to_string(*all));
        }
    }

    // Continued program logic goes here...
}
