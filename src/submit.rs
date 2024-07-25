use crate::Config;

pub fn submit(year: &str, day: &str, part: u8, cfg: &Config) {
    find_solution(&cfg, part);
    println!("Submitting for Day {day} of AoC {year}, Part: {part}");
}

fn find_solution(cfg: &Config, part: u8) -> Option<String> {
    let Config{year, day, lang: _, solutions} = cfg;
    if let Some(solved) = solutions.get(&format!("{year}-{day}")) {
        if let Some(submission) = solved.get((part - 1) as usize) {
            Some(submission.clone())
        } else {
            None
        }
    } else {
        None
    }
}
