use anyhow::Result;

pub fn print_result(roll: &str, result: Result<i32>) {
    match result {
        Ok(sum) => println!("\nRolling {roll}, result is: {sum}\n"),
        Err(_) => eprintln!(
            "\nError parsing input: {}\n\nPlease try again using dice notation.\n",
            roll
        ),
    }
}
