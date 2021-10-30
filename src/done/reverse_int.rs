fn main() {
    println!("{}", reverse_integer(123456));
}

fn reverse_integer(num: i32) -> i32 {
    let mut num_vec: Vec<f32> = Vec::new();
    {
        let mut cp = num;
        while cp > 0 {
            let f_cp: f32 = (cp as f32) / 10.0;
            cp /= 10;
            num_vec.push(((f_cp - (cp as f32)) * 10.0).round());
        }
    }
    let mut fin = 0;
    for (i, elem) in num_vec.iter().rev().enumerate() {
        println!("{}", i);
        fin += (10_i32.pow(i as u32)) * *elem as i32;
    }
    fin
}
