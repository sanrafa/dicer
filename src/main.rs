mod repl;
mod roll;
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
    Roll { roll: String },
}

#[derive(Debug)]
enum Mode<'a> {
    Interactive,
    Noninteractive(&'a Commands),
}

impl Mode<'_> {
    pub fn run(&self) {
        match self {
            Mode::Interactive => {
                repl::start().unwrap();
                println!("Goodbye!");
            }
            Mode::Noninteractive(cmd) => match cmd {
                Commands::Roll { roll } => {
                    let sum = roll::execute(roll);
                    roll::print_result(roll, sum);
                }
            },
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        None => Mode::Interactive.run(),
        Some(cmd) => Mode::Noninteractive(cmd).run(),
    }
}
