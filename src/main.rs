use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "dicer")]
#[command(about = "A dice roller CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Make a summed dice roll
    #[command(arg_required_else_help = true)]
    Roll { dice: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        None => println!("Entering interactive mode."),
        Some(cmd) => match cmd {
            Commands::Roll { dice } => println!("Rolling dice: {}", dice),
        },
    }
}
