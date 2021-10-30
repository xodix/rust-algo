#[macro_export]
macro_rules! include_file {
    ($p:expr) => {
        std::fs::read_to_string($p).unwrap()
    };
}

fn main() {}

#[cfg(test)]
mod test {
    #[test]
    fn read_res_json() {
        println!("{}", include_file!("res.json"));
    }
}
