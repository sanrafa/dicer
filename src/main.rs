use clap::{Parser, Subcommand};
use regex::Regex;

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
    let reg = Regex::new(r"(?P<total>\d+)d{1}(?P<faces>\d+)").unwrap();

    match &cli.command {
        None => println!("Entering interactive mode."),
        Some(cmd) => match cmd {
            Commands::Roll { dice } => {
                let roll = reg
                    .captures(&dice)
                    .expect("Input should use standard dice notation, i.e. 1d10");
                let (total, faces) = (&roll["total"], &roll["faces"]);
                println!("Rolling {total} {faces}-sided die.",);
            }
        },
    }
}
