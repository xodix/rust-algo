fn main() {
    print!(
        "{}",
        stock_list(
            vec!["AFDSF 150", "BDADF 11111", "XDDD 1"],
            vec!["A", "B", "C"]
        )
    );
}

use std::collections::HashMap;
fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    let mut cats: HashMap<char, usize> = HashMap::new();
    for cat in list_cat {
        cats.insert(cat.parse::<char>().unwrap(), 0usize);
    }
    for art in list_art {
        let mut art_ch = art.chars();
        let cat = art_ch.nth(0).unwrap();
        if !cats.contains_key(&cat) {
            continue;
        }
        let mut num_total = art_ch.nth_back(0).unwrap().to_digit(10).unwrap() as usize;
        println!("num: {}", num_total);
        let mut i = 1usize;
        loop {
            let num = art_ch.nth_back(0).unwrap();
            println!("num: {}, i: {}", num, i);
            if !num.is_numeric() {
                break;
            }
            num_total += num.to_digit(10).unwrap() as usize * 10usize.pow(i as u32);
            println!("num_total: {}", num_total);
            i += 1;
        }
        cats.insert(cat, *cats.get(&cat).unwrap() + num_total);
    }
    let mut fin = String::new();
    for (key, total) in cats.into_iter() {
        fin += &format!("({}: {}) - ", key, total);
    }
    fin.pop();
    fin.pop();
    fin.pop();
    fin
}
