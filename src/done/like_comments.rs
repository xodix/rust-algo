fn main() {
    let x = vec![
        vec![],
        vec!["Peter"],
        vec!["Peter", "Jacob"],
        vec!["Peter", "Jacob", "Alex"],
        vec!["Peter", "Jacob", "Alex", "Mark"],
        vec!["Peter", "Jacob", "Alex", "Mark", "Max"],
        vec!["Peter", "Jacob", "Alex", "Mark", "Max", "Jacob"],
    ];
    for elem in x {
        println!("{}\n\n", likes(elem))
    }
}

/**
    []                                -->  "no one likes this"
    ["Peter"]                         -->  "Peter likes this"
    ["Jacob", "Alex"]                 -->  "Jacob and Alex like this"
    ["Max", "John", "Mark"]           -->  "Max, John and Mark like this"
    ["Alex", "Jacob", "Mark", "Max"]  -->  "Alex, Jacob and 2 others like this"
*/
fn likes(names: Vec<&str>) -> String {
    let n = (&names).len();
    return match n {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        _ => format!("{}, {} and {} others like this", names[0], names[1], n - 2),
    };
}
