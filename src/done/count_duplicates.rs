use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    let mut dup_hash: HashMap<char, bool> = HashMap::new();
    let mut counter: u32 = 0;
    for s in text.chars() {
        if !dup_hash.contains_key(&s) {
            dup_hash.insert(s, false);
        } else if !*dup_hash.get(&s).unwrap() {
            counter += 1;
            dup_hash.insert(s, true);
        }
    }
    counter
}
