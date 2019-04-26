use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead};

mod constants;

use constants::*;

fn main() {
    let name_re = Regex::new(
        r"Name: (?P<name>.+) Training Program: (?P<training_program>\d+) ID Number: (?P<id>\d+)",
    )
    .unwrap();

    let mut trainees: HashMap<u32, Trainee> = HashMap::new();

    let mut trainee: Option<&mut Trainee> = None;

    let all_items: Vec<&str> = BASIC_SCIENCES_ITEMS
        .iter()
        .chain(CLINICAL_SCIENCES_ITEMS.iter())
        .chain(ORGAN_BASED_SCIENCES_ITEMS.iter())
        .chain(CLINICAL_SUBSPECIALTIES_ITEMS.iter())
        .chain(SPECIAL_PROBLEMS_ITEMS.iter())
        .map(|s| *s)
        .collect();

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line.as_ref() {
            if let Some(caps) = name_re.captures(line) {
                if let (Some(name), Some(_training_program), Some(id)) = (
                    caps.name("name"),
                    caps.name("training_program"),
                    caps.name("id"),
                ) {
                    let id = id.as_str().parse().unwrap();
                    let name = name.as_str().to_string();
                    trainee = Some(trainees.entry(id).or_insert(Trainee {
                        name,
                        id,
                        missed_topics: HashSet::new(),
                    }));
                }
            } else {
                if let Some(trainee) = trainee.as_mut() {
                    for item in all_items.iter() {
                        if line == *item {
                            trainee.missed_topics.insert(line.to_string());
                        }
                    }
                }
            }
        }
    }

    dbg!(&trainees);

    let mut trainees: Vec<Trainee> = trainees.drain().map(|(_, v)| v).collect();

    trainees.sort_unstable_by(|a, b| a.name.cmp(&b.name));

    dump_missed_topics(&trainees, &all_items).unwrap();
}

fn dump_missed_topics(trainees: &Vec<Trainee>, items: &Vec<&str>) -> Result<(), Box<Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_field("Name")?;
    writer.write_record(items.iter())?;

    for trainee in trainees.iter() {
        writer.write_field(&trainee.name)?;
        writer.write_record(items.iter().map(|item| {
            if trainee.missed_topics.contains(*item) {
                "x"
            } else {
                ""
            }
        }))?;
    }

    writer.flush()?;

    Ok(())
}

#[derive(Debug)]
struct Trainee {
    name: String,
    id: u32,
    missed_topics: HashSet<String>,
}
