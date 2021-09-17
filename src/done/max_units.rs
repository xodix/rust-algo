assert_eq!(
    maximum_units(&mut vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
    8
);
assert_eq!(
    maximum_units(
        &mut vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]],
        10
    ),
    91
);
// https://leetcode.com/problems/maximum-units-on-a-truck/
/*
Possible optimazations:
    - only sort until number of boxes exceeds trucksize
    - more pointers for vectors
*/
pub fn maximum_units(box_types: &mut Vec<Vec<i32>>, truck_size: i32) -> i32 {
    box_types.sort_by(|a, b| a[1].cmp(&b[1]));
    let mut sum = 0;
    let mut num_of_boxes = truck_size;
    // ! first check if number of boxes is in range
    for box_type in box_types.clone().iter().rev() {
        if num_of_boxes - box_type[0] <= 0 {
            break;
        } else {
            sum += box_type[1] * box_type[0];
            num_of_boxes -= box_type[0];
            box_types.pop();
        }
    }
    if num_of_boxes > 0 {
        // upb - units per box
        let curr_upb = box_types[box_types.len() - 1][1];
        sum += num_of_boxes * curr_upb;
    }
    return sum;
}
