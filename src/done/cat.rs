#[cfg(test)]
mod test {
    use crate::print_file;

    #[test]
    fn test_dbtxt_read() {
        print_file(&"db.txt".to_string()).expect("fail");
    }
}
use std::{env, fs, io::Error};
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    match print_file(filename) {
        Err(e) => println!(
            "Failed to read a file: {}\n Additional info: {}",
            filename, e
        ),
        Ok(_) => print!(""),
    };
}

fn print_file(f_name: &String) -> Result<(), Error> {
    let contents: String = fs::read_to_string(f_name)?;
    print!("{}", contents);
    Ok(())
}
