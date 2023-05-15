use anyhow::Result;
use rand::{distributions::Uniform, Rng};

pub fn roll_die(total: &str, faces: &str) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let x: i32 = total.parse().unwrap();
    let y: i32 = faces.parse().unwrap();
    let die = Uniform::from(1..=y);
    let throws: Vec<i32> = (0..x).map(|_| rng.sample(&die)).collect();
    throws
}

pub fn print_result(roll: &str, result: Result<i32>) {
    match result {
        Ok(sum) => println!("\nRolling {roll}, result is: {sum}\n"),
        Err(_) => eprintln!(
            "\nError parsing input: {}\n\nPlease try again using dice notation.\n",
            roll
        ),
    }
}
