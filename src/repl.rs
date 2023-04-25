use super::roll;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn start() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Entering interactive mode:");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.clone())?;
                let arg = line.as_str();
                let result = roll::execute(arg);
                roll::print_result(arg, result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
