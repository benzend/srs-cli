use clap::{Parser, Subcommand};
mod util;
use util::data_accessor;

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
        front: String,

        #[arg(short, long)]
        back: String,

        #[arg(short, long)]
        collection: String
    },
    Remove {
        #[arg(short, long)]
        ids: Vec<String>
    },
    List {
        #[arg(short, long)]
        verbose: bool,
    },
    Find {
        #[arg(short, long)]
        id: String
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
    Find {
        #[arg(short, long)]
        name: String,
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
                    let res = data_accessor::get_cards(None);
                    match res {
                        Ok(v) => {
                            for v in v {
                                println!("id: {}", v.id.unwrap_or(String::from("null")));
                                if *verbose {
                                    println!("front: {}", v.front.unwrap_or(String::from("null")));
                                }
                            }
                        }
                        Err(e) => println!("failed to get cards: {}", e)
                    }
                }
                CardCommands::Find { id } => {
                    let res = data_accessor::get_card(id.as_str());
                    match res {
                        Ok(v) => println!("front: {}, back: {}", v.front.unwrap_or(String::from("N/A")), v.back.unwrap_or(String::from("N/A"))),
                        Err(e) => println!("failed to find card: {}", e)
                    }
                }
                CardCommands::Add { front, back, collection } => {
                    let res = data_accessor::create_card(collection.as_str(), front.as_str(), back.as_str());
                    match res {
                        Ok(v) => println!("created card (id:{})", v),
                        Err(e) => println!("failed to create card: {}", e)
                    }
                }
                CardCommands::Update { id, front, back } => {
                    let res = data_accessor::update_card(id, front.as_deref(), back.as_deref());
                    match res {
                        Ok(_) => println!("updated card"),
                        Err(e) => println!("failed to update card: {}", e)
                    }
                }
                CardCommands::Remove { ids } => {
                    let res = data_accessor::remove_cards(ids.iter().map(AsRef::as_ref).collect());
                    match res {
                        Ok(_) => println!("removed cards"),
                        Err(e) => println!("failed to remove cards: {}", e)
                    }
                }
            }
        }
        Some(Subcommands::Collections { commands }) => {
            match commands {
                CollectionCommands::List { verbose } => {
                    let res = data_accessor::get_collections();
                    match res {
                        Ok(v) => {
                            for v in v {
                                println!("name: {}", v.name);
                                if *verbose {
                                    // TODO print more details, like card count, description if
                                    // there is, etc
                                }
                            }
                        }
                        Err(e) => println!("failed to get cards: {}", e)
                    }
                }
                CollectionCommands::Find { name } => {
                    let res = data_accessor::get_collection(name.as_str());
                    match res {
                        Ok(v) => println!("found collection! name: {}", v.name),
                        Err(e) => println!("failed to find collection: {}", e)
                    }
                }
                CollectionCommands::Add { name } => {
                    let res = data_accessor::create_collection(name.as_str());
                    match res {
                        Ok(_) => println!("created collection"),
                        Err(e) => println!("failed to create collection: {}", e)
                    }
                }
                CollectionCommands::Remove { name } => {
                    let res = data_accessor::remove_collection(name.as_str());
                    match res {
                        Ok(_) => println!("removed collection"),
                        Err(e) => println!("failed to remove collection: {}", e)
                    }
                }
                CollectionCommands::Review { name } => {
                    let res = data_accessor::review_collection(name.as_str());
                    match res {
                        Ok(_) => println!("reviewing collection..."),
                        Err(e) => println!("failed to review collection: {}", e)
                    }
                }
            }
        }
        _ => {}
    }
}
