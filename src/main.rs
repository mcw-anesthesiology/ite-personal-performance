use regex::Regex;

use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

mod constants;

use constants::*;

fn main() {
    let path = env::args().skip(1).next().unwrap();
    let contents = read_to_string(path).unwrap();

    let name_re = Regex::new(
        r"Name: (?P<name>.+) Training Program: (?P<training_program>\d+) ID Number: (?P<id>\d+)",
    )
    .unwrap();

    let mut trainees: HashMap<u32, Trainee> = HashMap::new();

    let mut trainee: Option<&mut Trainee> = None;

    let all_items: Vec<&&str> = BASIC_SCIENCES_ITEMS
        .iter()
        .chain(CLINICAL_SCIENCES_ITEMS.iter())
        .chain(ORGAN_BASED_SCIENCES_ITEMS.iter())
        .chain(CLINICAL_SUBSPECIALTIES_ITEMS.iter())
        .chain(SPECIAL_PROBLEMS_ITEMS.iter())
        .collect();

    for line in contents.lines() {
        if let Some(caps) = name_re.captures(&line) {
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
                    missed_topics: Vec::new(),
                }));
            }
        } else {
            if let Some(trainee) = trainee.as_mut() {
                for item in all_items.iter() {
                    if line == **item {
                        trainee.missed_topics.push(line.to_string());
                    }
                }
            }
        }
    }

    dbg!(&trainees);
}

#[derive(Debug)]
struct Trainee {
    name: String,
    id: u32,
    missed_topics: Vec<String>,
}
