pub fn calculate_fibonacci_peremeters(mut n_sec: u8) -> u128 {
  if n_sec > 183 {
    return 0;
  };
  let mut sum: u128 = 2;
  let mut n1: u128 = 1;
  let mut n2: u128 = 1;
  n_sec -= 1;
  for _i in 0..n_sec {
    let n2_tmp = n2;
    n2 = n1 + n2;
    n1 = n2_tmp;
    sum += n2;
  }
  sum * 4
}

pub fn fibonacii(mut n: u8) -> u128 {
  if n > 186 {
    return 0;
  };
  let mut n1: u128 = 1;
  let mut n2: u128 = 1;
  n -= 2;
  for _i in 0..n {
    let n2_tmp = n2;
    n2 = n1 + n2;
    n1 = n2_tmp;
  }
  n2
}
