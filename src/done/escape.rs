/**
# This is a html escaper
pls don't use for security purposes. I'm 18yo and I just made 10 mistakes writing this description.

it escapes '<', '>', '&', '"' and the '
*/
fn main() -> Result<(), std::io::Error> {
    let x = std::fs::read("res1.json")?;
    println!("{}", escape(x));
    Ok(())
}

// ASCII representation for escaped characters
const LESS_THAN: u8 = 0x3c;
const MORE_THAN: u8 = 0x3e;
const AMPERSAND: u8 = 0x26;
const SINGLE_QUOTE: u8 = 0x27;
const DOUBLE_QUOTE: u8 = 0x22;

pub fn escape(bytes: Vec<u8>) -> String {
    let mut fin = String::new();

    for byte in bytes {
        match byte {
            LESS_THAN => fin += "&lt;",
            MORE_THAN => fin += "&gt;",
            AMPERSAND => fin += "&amp;",
            SINGLE_QUOTE => fin += "&#39;",
            DOUBLE_QUOTE => fin += "&#34;",
            x => fin += &(x as char).to_string(),
        }
    }

    fin
}
