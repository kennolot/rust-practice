use std::{collections::HashMap};

fn main() {
    println!("multiples of= {}", solution(10)); 
    println!("pete the baker= {}", cakes(&HashMap::from([("flour", 500), ("sugar", 200), ("eggs", 1)]), &HashMap::from([("flour", 1200), ("sugar", 1200), ("eggs", 5), ("milk", 200)])));
    println!("bit counting= {}", count_bits(1234));
    println!("duplicate encoder= {}", duplicate_encode("din"));
    println!("count_duplicates= {}", count_duplicates("aabbcde"));
    println!("rot13= {}", rot13("test"));
    println!("make_readable= {}", make_readable(359999));
    println!("score= {}", score([5, 5, 1, 6, 5]));
}

fn solution(num: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    let mut sum: i32 = 0;
    for mult in 1..num {        
        if mult % 3 == 0 || mult % 5 == 0 {
            sum += mult;
        }
    }
    sum
}

fn cakes(recipe: &HashMap<&str, u32>, available: &HashMap<&str, u32>) -> u32 {    
    let mut max_cakes = u32::MAX;
    for (ingredient, required) in recipe {
        let available_amount = available.get(ingredient).unwrap_or(&0);
        let can_bake: u32 = available_amount / required;
        max_cakes = max_cakes.min(can_bake);
    }
    max_cakes
}

fn count_bits(n: i64) -> u32 {
    n.count_ones()
}

fn duplicate_encode(word: &str) -> String {
    let mut result = String::new();
    let lower_word = word.to_lowercase();

    for c in lower_word.chars() {
        if lower_word.matches(c).count() > 1 {
            result.push(')');
        } else {
            result.push('(');
        }
    }

    result
}

use std::collections::HashSet;
fn count_duplicates(text: &str) -> u32 {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();

    for c in text.to_lowercase().chars() {
        if !seen.insert(c) {
            duplicates.insert(c);
        }
    }

    duplicates.len() as u32
}


fn rot13(message: &str) -> String {
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];
    let upper_alphabet: Vec<_> = alphabet.iter().map(|c| c.to_ascii_uppercase()).collect();
 
    message.chars()
        .map(|c| *alphabet.iter()
        .chain(alphabet.iter())
        .chain(upper_alphabet.iter())
        .chain(upper_alphabet.iter())
        .skip_while(|&x| *x != c)
        .nth(13)
        .unwrap_or(&c))
        .collect()
}

fn make_readable(seconds: u32) -> String {    
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, secs)
}

fn get_pins(observed: &str) -> Vec<String> {
    let mut adj: HashMap<char, Vec<&str>> = HashMap::new();

    adj.insert('0', vec!["0", "8"]);
    adj.insert('1', vec!["1", "2", "4"]);
    adj.insert('2', vec!["1", "2", "3", "5"]);
    adj.insert('3', vec!["2", "3", "6"]);
    adj.insert('4', vec!["1", "4", "5", "7"]);
    adj.insert('5', vec!["2", "4", "5", "6", "8"]);
    adj.insert('6', vec!["3", "5", "6", "9"]);
    adj.insert('7', vec!["4", "7", "8"]);
    adj.insert('8', vec!["5", "7", "8", "9", "0"]);
    adj.insert('9', vec!["6", "8", "9"]);

    let mut possibilities = vec!["".to_string()];
    for dig in observed.chars() {
        if let Some(choices) = adj.get(&dig) {
            possibilities = possibilities.into_iter()
            .flat_map(|prefix| {
            choices.iter().map(move |&choice| format!("{}{}", prefix, choice))})
            .collect();
        }
    }
    possibilities
}

fn score(dice: [u8; 5]) -> u32 {
    let mut counts = HashMap::new();
    let mut points = 0;

    for &die in &dice {
        *counts.entry(die).or_insert(0) += 1;
    }        
    for (&value, &count) in &counts {
        points += match value {
            1 => {
                if count >= 3 {
                    1000 + (count - 3) as u32 * 100
                } else {
                    count as u32 * 100
                }
            },
            5 => {
                if count >= 3 {
                    500 + (count - 3) as u32 * 50
                } else {
                    count as u32 * 50
                }
            },
            2..=4 | 6 => {
                if count >= 3 {
                    value as u32 * 100
                } else {
                    0
                }
            },
            _ => 0,
        }
    }
    points
}