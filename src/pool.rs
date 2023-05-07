use super::roll;
use regex::Regex;

pub fn print_result(roll: &str, die_face: u16, results: Vec<(i32, bool)>) {
    let rolls: Vec<i32> = results.clone().into_iter().map(|(r, _)| r).collect();
    let successes: u32 = results
        .into_iter()
        .map(|(_, s)| if s { 1 } else { 0 })
        .sum();
    println!(
        "\nRolling pool {roll}, default die 1d{die_face}\nResult:\n\n{:?}\n\nSUCCESSES: {successes}\n",
        rolls
    )
}

pub fn execute(die_face: u16, roll: &str, t: Option<u16>) -> Vec<(i32, bool)> {
    let roll_reg = Regex::new(r"(?P<dice>\d+d\d+)|(?P<num>[0-9]+)").unwrap();
    let dice_reg = Regex::new(r"(?P<total>\d+)d{1}(?P<faces>\d+)").unwrap();

    fn possible_threshold(faces: u16, threshold: u16) -> bool {
        threshold < faces
    }

    fn meets_threshold(result: i32, threshold: u16) -> bool {
        return result >= threshold.into();
    }

    let matched: Vec<(i32, bool)> = roll_reg
        .captures_iter(roll)
        .map(|caps| {
            let dice_result = caps.name("dice").map(|d| {
                let dice = d.as_str();
                let dice_cap = dice_reg
                    .captures(dice)
                    .expect("Error parsing input. Please use standard dice notation.");
                let (total, faces) = (&dice_cap["total"], &dice_cap["faces"]);
                let max_die = faces.parse().unwrap();
                let threshold = t.unwrap_or(max_die / 2 + 1);
                if !possible_threshold(max_die, threshold) {
                    vec![(0, false); total.parse::<usize>().unwrap()]
                } else {
                    let throws = roll::roll_die(total, faces);
                    throws
                        .into_iter()
                        .map(|r| (r, meets_threshold(r, threshold)))
                        .collect()
                }
            });
            if dice_result.is_some() {
                dice_result.unwrap()
            } else {
                let num_result = caps.name("num").map(|n| {
                    let threshold = t.unwrap_or(die_face / 2 + 1);
                    if !possible_threshold(die_face, threshold) {
                        vec![(0, false); n.as_str().parse().unwrap()]
                    } else {
                        let num = n.as_str();
                        let throws = roll::roll_die(num, &die_face.to_string());
                        throws
                            .into_iter()
                            .map(|r| (r, meets_threshold(r, threshold)))
                            .collect()
                    }
                });
                num_result.unwrap_or(vec![(0, false)])
            }
        })
        .flatten()
        .collect();
    matched
}
