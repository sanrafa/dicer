use anyhow::{anyhow, Result};
use dicer_lib::pool;
use regex::Regex;

pub fn print_result(roll: &str, die_face: u16, results: Result<Vec<(i32, bool)>>) {
    match results {
        Ok(res) => {
            let rolls: Vec<i32> = res.clone().iter().map(|(r, _)| *r).collect();
            let successes: u32 = res.into_iter().map(|(_, s)| if s { 1 } else { 0 }).sum();
            println!(
                "\nRolling pool {roll}, default die 1d{die_face}\nResult:\n\n{:?}\n\nSUCCESSES: {successes}\n",
                rolls
            )
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn parse_threshold(t: Option<&str>) -> Result<f64> {
    if let Some(x) = t {
        if x.contains("/") {
            let frac_re = Regex::new(r"(?P<num>\d+)/(?P<den>\d+)")?;
            let caps = frac_re.captures(&x);
            if let Some(caps) = caps {
                let (num, den) = (&caps["num"], &caps["den"]);
                let threshold = num.parse::<f64>()? / den.parse::<f64>()?;
                Ok(threshold)
            } else {
                Err(anyhow!("parsing error"))
            }
        } else if x.contains(".") {
            let decimal = x.parse::<f64>()?;
            let threshold = decimal.round();
            Ok(threshold)
        } else {
            let num_re = Regex::new(r"^[0-9]+$")?;
            if num_re.is_match(&x) {
                let threshold = x.parse::<f64>()?;
                return Ok(threshold);
            }
            Err(anyhow!("parsing error"))
        }
    } else {
        Ok(0.5)
    }
}

pub fn execute(die_face: u16, roll: &str, t: Option<&str>) -> Result<Vec<(i32, bool)>> {
    if let Ok(thr) = parse_threshold(t) {
        let base = die_face as i32;
        pool(roll, base, thr)
    } else {
        Err(anyhow!(
            "Error parsing threshold. Must be a fraction, decimal, or integer."
        ))
    }
}
