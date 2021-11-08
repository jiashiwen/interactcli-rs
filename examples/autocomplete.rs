use serde::Deserialize;
use serde_json;
use std::fs;
use std::io;
use std::time;

#[derive(Deserialize, Debug)]
struct Case {
    typed: String,
    possibilities: Vec<String>,
}

fn main() -> Result<(), std::io::Error> {
    let cases: Vec<Case> =
        serde_json::from_reader(io::BufReader::new(fs::File::open("data.json")?))?;
    let before = time::Instant::now();
    for case in cases {
        autocomplete(&case.typed, &case.possibilities);
    }
    let time = before.elapsed();
    println!("{:?}", time);
    Ok(())
}

fn autocomplete(typed: &str, possibilities: &Vec<String>) -> String {
    let matches: Vec<&String> = possibilities
        .iter()
        .filter(|p| p.starts_with(typed))
        .collect();
    // If there are no matches, proceed to the next rule.
    if matches.len() == 0 {
        return autocomplete_contiguous(typed, possibilities);
    }
    // Scan the common prefix 1 char a time.
    for i in typed.len().. {
        let mut c = None;
        for m in &matches {
            if m.len() <= i || c != None && m.as_bytes()[i] != c.unwrap() {
                return m[..i].to_string();
            }
            if c == None {
                c = Some(m.as_bytes()[i])
            }
        }
    }
    typed.to_string()
}

fn autocomplete_contiguous(typed: &str, possibilities: &Vec<String>) -> String {
    let matches: Vec<&String> = possibilities.iter().filter(|p| p.contains(typed)).collect();
    if matches.len() == 1 {
        return matches[0].to_string();
    }
    if matches.len() > 1 {
        return typed.to_string();
    }
    autocomplete_in_order(typed, possibilities)
}

fn autocomplete_in_order(typed: &str, possibilities: &Vec<String>) -> String {
    let mut matching: Option<&String> = None;
    for s in possibilities {
        // See if we can find each character of typed in this possibility after where we found the last one.
        let mut lastmatch = 0;
        for (i, c) in typed.char_indices() {
            match s[lastmatch..].find(c) {
                None => break,
                Some(index) => {
                    lastmatch += index + 1;
                    // If we just found the last char.
                    if i + 1 == typed.len() {
                        // There was already a match. It's ambiguous.
                        if matching != None {
                            return typed.to_string();
                        }
                        matching = Some(s);
                    }
                }
            }
        }
    }
    match matching {
        None => typed.to_string(),
        Some(m) => m.to_string(),
    }
}
