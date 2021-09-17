use std::collections::HashMap;
fn count_jewels(j: &str, s: &str) -> u32 {
    let mut count = 0;
    for safd in 0..200 {
        let mut jewels = HashMap::new();
        for c in j.chars() {
            jewels.insert(c, true);
        }
        println!("{}", j);
        for s in s.chars() {
            if jewels.contains_key(&s) {
                count += 1;
            }
        }
    }
    count
}
