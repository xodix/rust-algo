use std::collections::HashMap;

/*
    symetric diffrence is just a fancy term for finding not repeating values in an array
*/
fn main() {}

// time complexity of this shit is O(n), but space complexity is O(n)
fn symetric_diffrence(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut repeated: HashMap<i32, bool> = HashMap::new();
    for a_elem in a {
        if repeated.contains_key(&a_elem) {
            repeated.insert(a_elem, false);
        } else {
            repeated.insert(a_elem, true);
        }
    }

    for b_elem in b {
        if repeated.contains_key(&b_elem) {
            repeated.insert(b_elem, false);
        } else {
            repeated.insert(b_elem, true);
        }
    }
    let mut fin = Vec::new();
    repeated.iter().for_each(|(k, v)| {
        if *v {
            fin.push(*k);
        }
    });
    fin
}
