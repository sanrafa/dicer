mod pool;
mod repl;
mod roll;
use clap::{Parser, Subcommand};
use dicer_lib::roll;

#[derive(Debug, Parser)]
#[command(name = "dicer")]
#[command(about = "A dice roller CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Default to dice pools. Can be overridden by prepending `roll` to command
    #[arg(short, long, default_value_t = false)]
    pool: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Make a summed dice roll
    #[command(arg_required_else_help = true)]
    Roll { roll: String },
    /// Roll a dice pool
    Pool {
        /// Set base number of die faces
        #[arg(short, long = "die", default_value_t = 10, value_name = "INTEGER")]
        dice_type: u16,
        /// Set threshold for a successful roll. If greater than 1, threshold is minimum for success
        #[arg(short, long, value_name = "FRACTION | DECIMAL | INTEGER")]
        threshold: Option<String>,
        /// Dice pool - can be dice notation or arithmetic. Returns result of each die
        roll: Option<String>,
    },
}

#[derive(Debug)]
enum Mode<'a> {
    Interactive(bool),
    Noninteractive(&'a Commands),
}

impl Mode<'_> {
    pub fn run(&self) {
        match self {
            Mode::Interactive(pool) => {
                repl::start(*pool).unwrap();
                println!("Goodbye!");
            }
            Mode::Noninteractive(cmd) => match cmd {
                Commands::Roll { roll } => {
                    let result = dicer_lib::roll(roll);
                    roll::print_result(roll, result)
                }
                Commands::Pool {
                    dice_type,
                    roll,
                    threshold,
                } => match roll {
                    None => eprintln!("Error: no roll argument provided. Use `dicer -p` or `dicer --pool` to enter the REPL in 'pool' mode."),
                    Some(roll) => {

                        let results = pool::execute(*dice_type, roll, threshold.as_deref());
                        pool::print_result(roll, *dice_type, results)
                    }
                },
            },
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        None => Mode::Interactive(cli.pool).run(),
        Some(cmd) => Mode::Noninteractive(cmd).run(),
    }
}
