fn can_see_person(heights: Vec<u32>) -> Vec<u32> {
    let n = (&heights).len();
    for i in 0..n {
        let mut people_seen = 0;
        let val = heights[i];
        for i in i..n {
            let h_val = heights[i];
            if !(h_val > val && heights[i + 1] < h_val) {
                people_seen += 1;
            }
        }
    }
    vec![1, 2, 3, 4]
}
