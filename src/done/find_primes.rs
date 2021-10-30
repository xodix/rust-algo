fn main() {
    print_without_zeros(find_primes_check());
}

pub fn create_100_vec() -> Vec<usize> {
    let mut res = Vec::with_capacity(100);

    for i in 1..101 {
        res.push(i);
    }

    res
}

pub fn print_without_zeros(primes: Vec<usize>) {
    primes.iter().for_each(|&i| {
        if i != 0 {
            println!("{}", i);
        }
    })
}

pub fn find_primes_exclusion() -> Vec<usize> {
    let mut numbers = create_100_vec();

    for i in 2..101 {
        if numbers[i] == 0 {
            continue;
        }
        let mut j = 2;

        while i * j <= 100 {
            numbers[i * j - 1] = 0;
            j += 1;
        }
    }

    numbers
}

pub fn find_primes_check() -> Vec<usize> {
    let mut numbers = create_100_vec();

    for i in 2..101 {
        if numbers[i - 1] == 0 {
            continue;
        }

        for j in 2..i {
            if i % j == 0 {
                numbers[i - 1] = 0;
            }
        }
    }

    numbers
}
