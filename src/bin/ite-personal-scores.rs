use regex::{Match, Regex};

use std::error::Error;
use std::io::{self, BufRead};

use ite_personal_performance::extract_header;

fn main() {
    let scaled_re = Regex::new(r"Your Scaled Score is: (?P<score>\d+)").unwrap();
    let basic_re =
        Regex::new(r"Your Percent Correct Score for Basic Items is: (?P<score>\d+)").unwrap();
    let advanced_re =
        Regex::new(r"Your Percent Correct Score for Advanced Items is: (?P<score>\d+)").unwrap();
    let category_re = Regex::new(
        r"(?P<category>\D+) (?P<total>\d+) (?P<correct>\d+) (?P<p50>\d+) (?P<p75>\d+) (?P<p90>\d+)",
    )
    .unwrap();

    let mut name: Option<String> = None;
    let mut id: Option<u32> = None;
    let mut scaled: Option<u32> = None;
    let mut basic: Option<u32> = None;
    let mut advanced: Option<u32> = None;

    let mut basic_sciences: Option<CategoryScore> = None;
    let mut clinical_sciences: Option<CategoryScore> = None;
    let mut organ_based_sciences: Option<CategoryScore> = None;
    let mut clinical_subspecialties: Option<CategoryScore> = None;
    let mut special_problems: Option<CategoryScore> = None;

    let mut trainees: Vec<Trainee> = Vec::new();

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line.as_ref() {
            if line == "THE AMERICAN BOARD OF ANESTHESIOLOGY" {
                match (name, id, scaled, basic, advanced) {
                    (Some(name), Some(id), Some(scaled), Some(basic), Some(advanced)) => {
                        trainees.push(Trainee {
                            name,
                            id,
                            scores: Scores {
                                scaled,
                                basic,
                                advanced,
                            },
                            category_scores: CategoryScores {
                                basic_sciences,
                                clinical_sciences,
                                organ_based_sciences,
                                clinical_subspecialties,
                                special_problems,
                            },
                        });
                    }
                    (Some(name), _, _, _, _) => {
                        eprintln!("failed to extract scores for {}", &name);
                    }
                    _ => {}
                }

                name = None;
                id = None;
                scaled = None;
                basic = None;
                advanced = None;

                basic_sciences = None;
                clinical_sciences = None;
                organ_based_sciences = None;
                clinical_subspecialties = None;
                special_problems = None;
            } else if let Some(caps) = extract_header(line) {
                if let (Some(name_cap), Some(id_cap)) = (caps.name("name"), caps.name("id")) {
                    id = id_cap.as_str().parse().ok();
                    name = Some(name_cap.as_str().to_string());
                } else {
                    id = None;
                    name = None;
                }
            } else if let Some(caps) = scaled_re.captures(line) {
                scaled = caps.name("score").and_then(match_to_u32);
            } else if let Some(caps) = basic_re.captures(line) {
                basic = caps.name("score").and_then(match_to_u32);
            } else if let Some(caps) = advanced_re.captures(line) {
                advanced = caps.name("score").and_then(match_to_u32);
            } else if let Some(caps) = category_re.captures(line) {
                if let (
                    Some(cat),
                    Some(total),
                    Some(correct),
                    Some(percentile_50),
                    Some(percentile_75),
                    Some(percentile_90),
                ) = (
                    caps.name("category"),
                    caps.name("total").and_then(match_to_u32),
                    caps.name("correct").and_then(match_to_u32),
                    caps.name("p50").and_then(match_to_u32),
                    caps.name("p75").and_then(match_to_u32),
                    caps.name("p90").and_then(match_to_u32),
                ) {
                    match cat.as_str() {
                        "Basic Sciences" => {
                            basic_sciences = Some(CategoryScore {
                                total,
                                correct,
                                percentile_50,
                                percentile_75,
                                percentile_90,
                            })
                        }
                        "Clinical Sciences" => {
                            clinical_sciences = Some(CategoryScore {
                                total,
                                correct,
                                percentile_50,
                                percentile_75,
                                percentile_90,
                            })
                        }
                        "Organ-based Basic & Clinical Sciences" => {
                            organ_based_sciences = Some(CategoryScore {
                                total,
                                correct,
                                percentile_50,
                                percentile_75,
                                percentile_90,
                            })
                        }
                        "Clinical Subspecialties" => {
                            clinical_subspecialties = Some(CategoryScore {
                                total,
                                correct,
                                percentile_50,
                                percentile_75,
                                percentile_90,
                            })
                        }
                        "Special Problems or Issues in Anesthesiology" => {
                            special_problems = Some(CategoryScore {
                                total,
                                correct,
                                percentile_50,
                                percentile_75,
                                percentile_90,
                            })
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    trainees.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    dump_scores(&trainees).unwrap();
}

fn match_to_u32(m: Match) -> Option<u32> {
    m.as_str().parse::<u32>().ok()
}

fn dump_scores(trainees: &Vec<Trainee>) -> Result<(), Box<Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_record(
        ["Name", "Scaled", "Basic", "Advanced"]
            .iter()
            .map(|s| s.to_string())
            .chain(
                [
                    "Basic Sciences",
                    "Clinical Sciences",
                    "Organ-based Basic & Clinical Sciences",
                    "Clinical Subspecialties",
                    "Special Problems or Issues in Anesthesiology",
                ]
                .iter()
                .flat_map(move |category| {
                    ["# questions", "# correct", "50%-ile", "75%-ile", "90%-ile"]
                        .iter()
                        .map(move |field| format!("{} {}", category, field))
                }),
            ),
    )?;

    for trainee in trainees.iter() {
        writer.write_record(
            vec![
                trainee.name.clone(),
                trainee.scores.scaled.to_string(),
                trainee.scores.basic.to_string(),
                trainee.scores.advanced.to_string(),
            ]
            .into_iter()
            .chain(
                [
                    &trainee.category_scores.basic_sciences,
                    &trainee.category_scores.clinical_sciences,
                    &trainee.category_scores.organ_based_sciences,
                    &trainee.category_scores.clinical_subspecialties,
                    &trainee.category_scores.special_problems,
                ]
                .iter()
                .flat_map(|cs| {
                    if let Some(cs) = cs {
                        vec![
                            cs.total.to_string(),
                            cs.correct.to_string(),
                            cs.percentile_50.to_string(),
                            cs.percentile_75.to_string(),
                            cs.percentile_90.to_string(),
                        ]
                    } else {
                        vec![
                            "".to_string(),
                            "".to_string(),
                            "".to_string(),
                            "".to_string(),
                            "".to_string(),
                        ]
                    }
                }),
            ),
        )?;
    }

    Ok(())
}

#[derive(Debug)]
struct Trainee {
    name: String,
    id: u32,
    scores: Scores,
    category_scores: CategoryScores,
}

#[derive(Debug)]
struct Scores {
    scaled: u32,
    basic: u32,
    advanced: u32,
}

#[derive(Debug)]
struct CategoryScores {
    basic_sciences: Option<CategoryScore>,
    clinical_sciences: Option<CategoryScore>,
    organ_based_sciences: Option<CategoryScore>,
    clinical_subspecialties: Option<CategoryScore>,
    special_problems: Option<CategoryScore>,
}

#[derive(Debug)]
struct CategoryScore {
    total: u32,
    correct: u32,
    percentile_50: u32,
    percentile_75: u32,
    percentile_90: u32,
}
