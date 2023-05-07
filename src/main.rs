mod pool;
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
    /// Roll a dice pool
    Pool {
        /// Number of die faces. Ignored if provided in roll argument
        #[arg(short, long = "dice", default_value_t = 10, value_name = "NUMBER")]
        dice_type: u16,
        /// Dice pool - can be dice notation or arithmetic. Returns result of each die.
        roll: Option<String>,
    },
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
                Commands::Pool { dice_type, roll } => match roll {
                    None => println!("Error: no roll argument provided."),
                    Some(roll) => {
                        let result = pool::execute(*dice_type, roll);
                        pool::print_result(roll, *dice_type, result);
                    }
                },
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
