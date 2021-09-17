pub fn total_money(n: i32) -> i32 {
    let mut sum = 0;
    let num_of_weeks = n / 7;
    if num_of_weeks > 0 {
        // quick count by weeks
        for i in 1..=(n / 7) {
            sum += 28 + (7 * (i - 1));
        }
    }
    // count up the rest of the days
    for i in 1..=(n % 7) {
        sum += i + num_of_weeks;
    }
    sum
}

// assert_eq!(total_money(4), 10);
// assert_eq!(total_money(10), 37);
// assert_eq!(total_money(20), 96);
