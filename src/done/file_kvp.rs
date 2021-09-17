use std::fs::read_to_string;
use std::{collections::HashMap, io::Error};

fn load_file(filename: &str) -> Result<HashMap<String, String>, Error> {
    let mut res_map: HashMap<String, String> = HashMap::new();
    let content = read_to_string(filename)?;
    let arr: Vec<&str> = content.split("\r\n").collect();
    let mut kvp: Vec<Vec<&str>> = vec![];
    for a in arr {
        kvp.push(a.split(" ").collect());
    }
    for e in kvp {
        res_map.insert(e[0].to_string(), e[1].to_string());
    }
    Ok(res_map)
}

println!("{:?}", load_file("db.txt").unwrap());
