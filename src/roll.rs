use rand::{distributions::Uniform, Rng};
use regex::Regex;

pub fn roll_die(total: &str, faces: &str) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let x: i32 = total.parse().unwrap();
    let y: i32 = faces.parse().unwrap();
    let die = Uniform::from(1..=y);
    let throws: Vec<i32> = (0..x).map(|_| rng.sample(&die)).collect();
    throws
}

fn sum_die(total: &str, faces: &str) -> i32 {
    let throws = roll_die(total, faces);
    let sum = throws.iter().sum();
    sum
}

pub fn print_result(roll: &str, sum: i32) {
    if sum == 0 {
        println!("\nError parsing input. Please try again using dice notation.\n");
    } else {
        println!("\nRolling {roll}, result is: {sum}\n");
    }
}

pub fn execute(roll_arg: &str) -> i32 {
    let dice_reg = Regex::new(r"(?P<total>\d+)d{1}(?P<faces>\d+)").unwrap();
    let cmd_reg = Regex::new(r"((?P<dice>[+-]?\d+d\d+)|(?P<num>[+-]\d+))").unwrap();
    let matches = cmd_reg.captures_iter(roll_arg);
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
                -sum
            } else {
                sum
            }
        } else if num.len() > 0 {
            let num = num.replace("+", "").parse::<i32>().unwrap();
            num
        } else {
            0
        }
    });
    let sum: i32 = results.sum();
    sum
}
