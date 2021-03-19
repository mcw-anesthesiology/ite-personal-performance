use regex::Regex;

use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    io::{self, BufRead},
    iter::IntoIterator,
};

use ite_personal_performance::extract_header;

fn main() {
    let item_re: Regex = Regex::new(r".+ \([A-B]\)$").unwrap();

    let mut trainees: HashMap<u32, Trainee> = HashMap::new();

    let mut trainee: Option<&mut Trainee> = None;

    let mut all_items: HashSet<String> = HashSet::new();

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line.as_ref() {
            if let Some(caps) = extract_header(line) {
                if let (Some(name), Some(id)) = (caps.name("name"), caps.name("id")) {
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
                    if item_re.is_match(line) {
                        let line = line.to_string();
                        if !all_items.contains(&line) {
                            all_items.insert(line.clone());
                        }
                        trainee.missed_topics.insert(line);
                    }
                }
            }
        }
    }

    let mut trainees: Vec<Trainee> = trainees.drain().map(|(_, v)| v).collect();

    trainees.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    let mut all_items: Vec<String> = all_items.into_iter().collect();
    all_items.sort_unstable();

    if env::args().any(|arg| arg == "--by-topic") {
        if env::args().any(|arg| arg == "--list") {
            dump_trainee_topics_list(&trainees, all_items.as_slice()).unwrap();
        } else {
            dump_trainee_topics(&trainees, all_items.as_slice()).unwrap();
        }
    } else {
        dump_missed_topics(&trainees, all_items.as_slice()).unwrap();
    }
}

fn dump_missed_topics(trainees: &Vec<Trainee>, items: &[String]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_field("Name")?;
    writer.write_record(items.iter())?;

    for trainee in trainees.iter() {
        writer.write_field(&trainee.name)?;
        writer.write_record(items.iter().map(|item| {
            if trainee.missed_topics.contains(item) {
                "x"
            } else {
                ""
            }
        }))?;
    }

    writer.flush()?;

    Ok(())
}

fn dump_trainee_topics(trainees: &Vec<Trainee>, items: &[String]) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.write_field("Item")?;
    writer.write_record(trainees.iter().map(|t| &t.name))?;

    for item in items {
        writer.write_field(&item)?;
        writer.write_record(trainees.iter().map(|trainee| {
            if trainee.missed_topics.contains(item) {
                "x"
            } else {
                ""
            }
        }))?;
    }

    writer.flush()?;
    Ok(())
}

fn dump_trainee_topics_list(
    trainees: &Vec<Trainee>,
    items: &[String],
) -> Result<(), Box<dyn Error>> {
    for item in items {
        println!("## {}\n", item);
        for trainee in trainees {
            if trainee.missed_topics.contains(item) {
                println!("- {}", trainee.name);
            }
        }
        println!();
    }

    Ok(())
}

#[derive(Debug)]
struct Trainee {
    name: String,
    id: u32,
    missed_topics: HashSet<String>,
}
