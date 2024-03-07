use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
#[command(version, about, long_about = None)]
enum Subcommands {
    Cards {
        #[command(subcommand)]
        commands: CardCommands
    },
    Collections {
        #[command(subcommand)]
        commands: CollectionCommands
    }
}

#[derive(Subcommand, Debug)]
enum CardCommands {
    Add {
        #[arg(short, long)]
        front: Option<String>,

        #[arg(short, long)]
        back: Option<String>,

        #[arg(short, long)]
        collection: String
    },
    Remove {
        #[arg(short, long)]
        ids: String
    },
    List {
        #[arg(short, long)]
        verbose: bool,
    },
    Update {
        #[arg(short, long)]
        id: String,

        #[arg(short, long)]
        front: Option<String>,

        #[arg(short, long)]
        back: Option<String>
    },
}

#[derive(Subcommand, Debug)]
enum CollectionCommands {
    Add {
        /// add a collection, the name is used as a string
        #[arg(short, long)]
        name: String
    },
    List {
        #[arg(short, long)]
        verbose: bool
    },
    Remove {
        #[arg(short, long)]
        name: String
    },
    Review {
        #[arg(short, long)]
        name: String
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.commands {
        Some(Subcommands::Cards { commands }) => {
            match commands {
                CardCommands::List { verbose } => {
                    println!("TODO: List Cards");
                }
                CardCommands::Add { front, back, collection } => {
                    println!("TODO: Add Card");
                }
                CardCommands::Update { id, front, back } => {
                    println!("TODO: Update Card");
                }
                CardCommands::Remove { ids } => {
                    println!("TODO: Remove Card");
                }
            }
        }
        Some(Subcommands::Collections { commands }) => {
            match commands {
                CollectionCommands::List { verbose } => {
                    println!("TODO: List Collections");
                }
                CollectionCommands::Add { name } => {
                    println!("TODO: Add Collection");
                }
                CollectionCommands::Remove { name } => {
                    println!("TODO: Remove Collection");
                }
                CollectionCommands::Review { name } => {
                    println!("TODO: Remove Collection");
                }
            }
        }
        _ => {}
    }
}
