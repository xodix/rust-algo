use std::collections::HashMap;

fn main() {
    if good_pairs(vec![1, 2, 3, 1, 1, 3]) == 4 {
        println!("OK great job");
    } else {
        println!("{}", good_pairs(vec![1, 2, 3, 1, 1, 3]));
    }
}

fn good_pairs(nums: Vec<i32>) -> i32 {
    let mut already_map: HashMap<i32, bool> = HashMap::new();
    let mut pairs = 0;
    for item in nums {
        if *already_map.get(&item).unwrap_or(&false) {
            pairs += 1;
        } else {
            already_map.insert(item, true);
        }
        println!("{:?}", already_map);
    }
    pairs
}

// nums[i] == nums[j] i < j
// x = [1, 1, 1, 1].len() - 1 {3}
// 3+2+1
