use clap::{Parser, Subcommand};

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
        project: Option<String>,
    },
    Backburner {
        id: u32,
    },
    Busy {
        id: u32,
    },
    Clean,
    Clear,
    Done {
        id: u32,
    },
    List,
    Undo {
        id: u32,
    },
    Xp,
}

fn main() {
    color_eyre::install().unwrap();
    recall::apply_daily_penalty();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add {
            name,
            priority,
            project,
        } => {
            cli::add::add(name.to_string(), *priority, project.clone());
        }
        Commands::Backburner { id } => cli::backburner::backburner(*id),
        Commands::Busy { id } => cli::busy::busy(*id),
        Commands::Clean => cli::clean::clean(),
        Commands::Clear => {
            println!("called 'clear'");
        }
        Commands::Done { id } => cli::done::done(*id),
        Commands::List => cli::list::list(),
        Commands::Undo { id } => cli::undo::undo(*id),
        Commands::Xp => {
            println!("called 'xp'");
        }
    }
}
