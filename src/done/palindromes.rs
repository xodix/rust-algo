fn main() {
    println!("{}", is_palindrome(1000000001));
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut num_arr: Vec<f64> = vec![];
    {
        let mut num_cp: f64 = x as f64;
        while num_cp >= 1. {
            num_cp /= 10.;
            let curr = (num_cp - num_cp.floor()) * 10.;
            num_arr.push(curr.round());
            println!("{} {}", curr, curr.round());
            num_cp = num_cp.floor();
        }
    }
    let mut num_reverse = num_arr.clone();
    num_reverse.reverse();
    println!("{:?} {:?}", num_arr, num_reverse);
    if num_arr == num_reverse {
        true
    } else {
        false
    }
}
