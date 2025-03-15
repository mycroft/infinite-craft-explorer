use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Element {
    emoji: Option<String>,
    name: Option<String>,
    cost: i32,
    other1: Option<String>,
    other2: Option<String>,

    #[serde(default)]
    key: Option<String>,
}

fn find_element(
    elements: &HashMap<String, Element>,
    known_elements: &Vec<String>,
    name: &str,
) -> String {
    let record = elements.get(name);
    if record.is_none() {
        panic!("Element not found");
    }

    let record = record.unwrap();

    if record.cost == 1 || known_elements.contains(&String::from(record.name.clone().unwrap())) {
        record.name.clone().unwrap()
    } else {
        format!(
            "({} + {})",
            find_element(elements, known_elements, &record.other1.clone().unwrap()),
            find_element(elements, known_elements, &record.other2.clone().unwrap())
        )
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    // Read the JSON file
    let file_content = fs::read_to_string("data/index.json").expect("Unable to read file");

    // Get the known elements
    let known_elements: Vec<String> = fs::read_to_string("known_elements.txt")
        .map(|contents| contents.lines().map(String::from).collect())
        .unwrap_or_else(|err| {
            eprintln!("Failed to read known_elements.txt: {}", err);
            Vec::new()
        });

    // Parse the JSON content
    let mut parsed: HashMap<String, Element> =
        serde_json::from_str(&file_content).expect("Unable to parse JSON");

    if args.len() == 2 {
        let res = find_element(&parsed, &known_elements, &args[1]);
        println!("{}", res);
    } else {
        let mut all_elements = Vec::new();
        let max_cost = parsed.values().map(|e| e.cost).max().unwrap();
        // Dump all elements
        for (key, element) in parsed.iter_mut() {
            element.key = Some(key.clone());
            // let name = element.name.clone().unwrap();
            // let a = element.other1.clone().unwrap();
            // let b = element.other2.clone().unwrap();
            // println!(
            //     "{} -> {} (cost: {}: {} + {})",
            //     key, name, element.cost, a, b
            // );

            all_elements.push(element.clone());
        }

        let mut all_elements_sorted = all_elements.clone();
        all_elements_sorted.sort_by(|a, b| a.cost.cmp(&b.cost));

        println!("All elements:");
        for element in all_elements_sorted {
            println!(
                "{} -> {} (cost: {})",
                element.key.unwrap(),
                element.name.unwrap(),
                element.cost
            );
        }

        println!("Max cost: {}", max_cost);
    }
}
