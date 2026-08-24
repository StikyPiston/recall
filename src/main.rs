use clap::{Parser, Subcommand};

mod help;
mod cli;

static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "recall")]
#[command(version = &VERSION)]
#[command(about = "A minimal to-do list with a few amenities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        #[arg(value_parser = clap::value_parser!(u8).range(1..=3))]
        priority: u8,
        #[arg(short, long)]
        project: Option<String>
    },
    Backburner { id: u32 },
    Busy { id: u32 },
    Clean,
    Clear,
    Done { id: u32 },
    List,
    Undo { id: u32 },
    Xp,
}

fn main() {
    color_eyre::install().unwrap();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name, priority, project } => {
            println!(
                "called 'add' with name as {}, priority as {}, and project as {:#?}",
                name, priority, project
            );
        }
        Commands::Backburner { id } => {
            println!("called 'backburner' with id as {}", id);
        }
        Commands::Busy { id } => {
            println!("called 'busy' with id as {}", id);
        }
        Commands::Clean => {
            println!("called 'clean'");
        }
        Commands::Clear => {
            println!("called 'clear'");
        }
        Commands::Done { id } => {
            println!("called 'done' with id as {}", id);
        }
        Commands::List => {
            println!("called 'list'");
        }
        Commands::Undo { id } => {
            println!("called 'undo' with id of {}", id);
        }
        Commands::Xp => {
            println!("called 'xp'");
        }
    }
}
