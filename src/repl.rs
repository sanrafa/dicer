use super::pool;
use super::roll;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn start(pool: bool) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Entering interactive mode:");
    if pool {
        println!("\nPOOL mode activated.\n")
    };
    let mut pool_enabled = pool;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.clone())?;
                let arg = line.as_str();
                if !pool_enabled {
                    if arg == "pool" {
                        pool_enabled = true;
                        println!("\nPool mode activated. Returning pooled results:\n")
                    } else {
                        let result = dicer_lib::roll(arg);
                        roll::print_result(arg, result)
                    }
                } else {
                    if arg.starts_with("roll") {
                        let roll = arg.split_once("roll ");
                        match roll {
                            None => {
                                pool_enabled = false;
                                println!("\nPool mode deactivated. Returning summed rolls:\n")
                            }
                            Some(arg) => {
                                let (_, roll) = arg;
                                let result = dicer_lib::roll(roll);
                                roll::print_result(roll, result)
                            }
                        }
                    } else {
                        /*
                           TODO: Allow user to control threshold. Currently, it'll default to half + 1 of die max.
                        */
                        if arg.is_empty() {
                            eprintln!("\nError: No argument provided.\n");
                        } else {
                            let default_die = 10;
                            let results = pool::execute(default_die, arg, None);
                            pool::print_result(arg, default_die, results);
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
