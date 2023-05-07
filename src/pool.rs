use super::roll;
use regex::Regex;

pub fn print_result(roll: &str, die_face: u16, result: Vec<i32>) {
    println!(
        "Rolling pool {roll}, default die 1d{die_face}\nResult:\n\n{:?}",
        result
    )
}

pub fn execute(die_face: u16, roll: &str) -> Vec<i32> {
    let roll_reg = Regex::new(r"(?P<dice>\d+d\d+)|(?P<num>[0-9]+)").unwrap();
    let dice_reg = Regex::new(r"(?P<total>\d+)d{1}(?P<faces>\d+)").unwrap();

    let matched: Vec<i32> = roll_reg
        .captures_iter(roll)
        .map(|caps| {
            let dice_result = caps.name("dice").map(|d| {
                let dice = d.as_str();
                let dice_cap = dice_reg
                    .captures(dice)
                    .expect("Error parsing input. Please use standard dice notation.");
                let (total, faces) = (&dice_cap["total"], &dice_cap["faces"]);
                let throws = roll::roll_die(total, faces);
                throws
            });
            if dice_result.is_some() {
                dice_result.unwrap()
            } else {
                let num_result = caps.name("num").map(|n| {
                    let num = n.as_str();
                    let result = roll::roll_die(num, &die_face.to_string());
                    result
                });
                num_result.unwrap_or(vec![0])
            }
        })
        .flatten()
        .collect();
    matched
}
