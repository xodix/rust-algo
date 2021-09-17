print!("{:?}", convert_digits_to_vector(233));
println!("{}", persistance(999));

pub fn persistance(mut num: u64) -> u64 {
  let mut i = 0;
  loop {
      if is_digit(num) {
          break;
      }
      let v = convert_digits_to_vector(num);
      num = multiply_vector(v);
      i += 1;
  }
  i
}

fn convert_digits_to_vector(num: u64) -> Vec<u64> {
  let mut num_vector = vec![];
  let mut operative_float = num as f64;
  loop {
      println!("{} \n {:?}", operative_float, num_vector);
      if operative_float < 0.1 {
          num_vector.reverse();
          return num_vector;
      }
      num_vector.push(((operative_float / 10. - (operative_float / 10.).floor()) * 10.) as u64);
      operative_float = (operative_float / 10.).floor();
  }
}

fn is_digit(num: u64) -> bool {
  (num / 10) < 1
}

fn multiply_vector(arr: Vec<u64>) -> u64 {
  let mut sum = 1;
  for num in arr {
      sum *= num;
  }
  sum
}
