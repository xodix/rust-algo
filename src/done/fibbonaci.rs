use std::time::Instant;
fn main() {
    let time = Instant::now();
    println!("Running fibbonaci function...\n\n");
    fibbonaci(183);
    let elapsed = time.elapsed();
    println!("\n\nExecution time of fibbonaci function is {:?}", elapsed);
}

fn fibbonaci(n: u64) -> u128 {
    if n > 183 {
        panic!(
            "cannot create bigger fibbonaci sequence than n = 183 becouse it results in overflow"
        );
    }
    let mut first = 1;
    let mut second = 1;
    for _ in 0..n {
        let temp_first = first;
        first = second;
        second = temp_first + first;
        // println!("{} - {}", second, i);
    }
    second
}
// fn houndred_doors() -> [bool; 100] {
//     let mut door_states: [bool; 100] = [false; 100];
//     for i in 1..100 {
//         for j in (0..100).step_by(i) {
//             let j_door_state = door_states[j];
//             if j_door_state {
//                 door_states[j] = false;
//             } else {
//                 door_states[j] = true;
//             }
//         }
//     }
//     door_states
// }
