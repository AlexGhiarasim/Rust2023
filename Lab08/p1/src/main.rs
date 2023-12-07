use anyhow::Result;
use std::collections::HashMap;
use std::fs; // added item in dependencies

fn main() -> Result<()> {
    let content = fs::read_to_string("file.txt")?;
    let all_words: Vec<&str> = content
        .split(|c: char| !c.is_alphanumeric())
        .flat_map(|part| part.split_whitespace())
        .filter(|&word| !word.is_empty())
        .collect();

    let mut word_count: HashMap<String, i32> = HashMap::new();
    for word in all_words.iter() {
        *word_count.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let mut sorted_words: Vec<(String, i32)> = word_count.into_iter().collect(); // sorted by frecv and alphanumeric
    sorted_words.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let max_word_length = sorted_words
        .iter()
        .map(|(word, _)| word.len())
        .max()
        .unwrap_or(0);

    for (word, count) in &sorted_words {
        //print
        println!("{:<width$} => {}", word, count, width = max_word_length);
    }

    Ok(())
}
