assert_eq!(len(123), 3);
assert_eq!(num_to_vec(123), vec![1, 2, 3]);
assert_eq!(funny_n_things(123, 1), 90);

fn len(mut num: i32) -> i32 {
    let mut i = 0;
    while num >= 1 {
        num /= 10;
        i += 1;
    }
    i
}

fn num_to_vec(num: i32) -> Vec<i32> {
    let mut n = num as f32;
    let mut n_arr: Vec<i32> = vec![];
    while n >= 1. {
        n /= 10.;
        n_arr.push(((n - n.floor()) * 10.) as i32);
        n = n.floor();
    }
    n_arr.reverse();
    n_arr
}

fn funny_n_things(num: i32, a: u32) -> i32 {
    let mut n = num as f32;
    let mut n_arr: Vec<i32> = vec![];
    while n >= 1. {
        n /= 10.;
        n_arr.push(((n - n.floor()) * 10.) as i32);
        n = n.floor();
    }
    n_arr.reverse();
    let mut sum = 0;
    for (i, nu) in n_arr.iter().enumerate() {
        sum += nu.pow(i as u32 + a + 1);
    }
    sum
}
