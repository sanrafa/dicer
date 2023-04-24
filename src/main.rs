use clap::{Parser, Subcommand};
use rand::{distributions::Uniform, Rng};
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
    Roll { roll: String },
}

#[derive(Debug)]
enum Mode<'a> {
    Interactive,
    Noninteractive(&'a Commands),
}

impl Mode<'_> {
    pub fn run(&self) {
        let dice_reg = Regex::new(r"(?P<total>\d+)d{1}(?P<faces>\d+)").unwrap();
        let re = Regex::new(r"((?P<dice>[+-]?\d+d\d+)|(?P<num>[+-]\d+))").unwrap();
        match self {
            Mode::Interactive => println!("Entering interactive mode."),
            Mode::Noninteractive(cmd) => match cmd {
                Commands::Roll { roll } => {
                    let matches = re.captures_iter(roll);
                    let results = matches.map(|caps| {
                        let dice = match caps.name("dice") {
                            None => "",
                            Some(x) => x.as_str(),
                        };
                        let num = match caps.name("num") {
                            None => "",
                            Some(x) => x.as_str(),
                        };
                        if dice.len() > 0 {
                            let is_neg: bool;
                            let mut dice_chars = dice.clone().chars();
                            let first_char = dice_chars.next().unwrap();
                            if first_char.to_string() == "-" {
                                is_neg = true;
                            } else {
                                is_neg = false;
                            }
                            let roll_cap = dice_reg
                                .captures(&dice)
                                .expect("Argument should be standard dice notation.");
                            let (total, faces) = (&roll_cap["total"], &roll_cap["faces"]);
                            let sum = sum_die(total, faces);
                            if is_neg == true {
                                println!("Dice: {}, result: {}", dice, -sum);
                                -sum
                            } else {
                                println!("Dice: {}, result: {}", dice, sum);
                                sum
                            }
                        } else if num.len() > 0 {
                            let num = num.replace("+", "").parse::<i32>().unwrap();
                            println!("Number: {num}");
                            num
                        } else {
                            0
                        }
                    });
                    let sum: i32 = results.sum();
                    println!("Rolling {roll}, result is: {sum}");
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

fn sum_die(total: &str, faces: &str) -> i32 {
    let mut rng = rand::thread_rng();
    let x: i32 = total.parse().unwrap();
    let y: i32 = faces.parse().unwrap();
    let die = Uniform::from(1..=y);
    let throws: Vec<i32> = (0..x).map(|_| rng.sample(&die)).collect();

    let sum = throws.iter().sum();
    return sum;
}
