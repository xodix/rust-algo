use std::collections::HashMap;
fn main() {}

fn two_sum(nums: Vec<i32>, target: i32) -> (i32, i32) {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if m.contains_key(&(target - num)) {
            return (*m.get(&(target - num)).unwrap() as i32, i as i32);
        }
        m.insert(*num, i);
    }
    (-1, -1)
}

mod tests {
    use crate::two_sum;
    #[test]
    fn unit_tests() {
        assert_eq!(two_sum(vec![1, 2, 3, 4, 5, 6], 6), (1, 3));
        assert_eq!(two_sum(vec![1, 2, 3, 4, 5, 6, 7, 8], 230), (-1, -1));
    }
}
