fn main() {
    let x = "PAYPALISHIRING";
    println!("{}", convert(x.to_string(), 3));
}

fn convert(s: String, num_rows: i32) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut chars = s.chars();
    for _i in 0..num_rows {
        result.push(chars.nth(0).unwrap().to_string());
    }
    for i in num_rows as usize..s.len() {
        result[i % 3] += &chars.nth(0).unwrap().to_string();
    }
    let mut fin = "".to_string();
    for i in 0..result.len() {
        fin += &result[i];
    }
    return fin;
}
