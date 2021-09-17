/// Brainfuck compiler
use std::{env, io};
fn main() {
    let path = env::args().nth(1).expect("File name not given");
    let source = load_source(path);
    run(source);
}

fn run(source: Vec<u8>) {
    //initializing brainfuck commands as consts
    /// '>' move pointer right
    const MVR: u8 = b'>';
    /// '<' move pointer left
    const MVL: u8 = b'<';
    /// '+' increment position
    const ADD: u8 = b'+';
    /// '-' decrement position
    const SUB: u8 = b'-';
    /// '.' display value on position (ASCII number)
    const DIS: u8 = b'.';
    /// ',' gets a value from user and sets it on position
    const INP: u8 = b',';
    /// '[' jumps to corresponding ] if 0 is set on position
    const JIF: u8 = b'[';
    /// ']' jumps to corresponding [
    const EIF: u8 = b']';
    let mut table: Vec<u8> = vec![0; 30_000];
    let mut pointer: usize = 0;

    let mut command_id = 0;

    let mut jif_positions: Vec<usize> = Vec::new();

    while command_id < source.len() {
        let command = source[command_id];

        match command {
            MVR => pointer += 1,
            MVL => pointer -= 1,
            ADD => table[pointer] = table[pointer].overflowing_add(1).0,
            SUB => table[pointer] = table[pointer].overflowing_sub(1).0,
            DIS => print!("{}", table[pointer] as char),
            INP => table[pointer] = get_user_input(),
            JIF => {
                /*
                if value on position is 0 then skip to corresponding ']'
                in other circumstances continue with execution
                */
                if table[pointer] == 0 {
                    command_id = find_closest_eif(command_id, &source);
                } else {
                    jif_positions.push(command_id);
                }
            }
            EIF => {
                /*
                if value is 0 move continue with execution
                in other circumstances pop last '[' position
                */
                if table[pointer] != 0 {
                    // we set command_id to JIF but it's added later. Look => 69:9
                    command_id = jif_positions[jif_positions.len() - 1];
                } else {
                    jif_positions.pop();
                }
            }
            _ => (),
        }

        command_id += 1;
    }
}

fn load_source(path: String) -> Vec<u8> {
    std::fs::read(path).expect("can't read source file")
}

fn get_user_input() -> u8 {
    loop {
        let mut buff = "".to_string();
        io::stdin().read_line(&mut buff).unwrap();
        if buff.len() != 3 {
            println!(
                "\n You must input one character. You inputed {}",
                buff.len() - 3
            );
        } else {
            return buff.bytes().nth(0).unwrap();
        }
    }
}

fn find_closest_eif(mut id: usize, source: &Vec<u8>) -> usize {
    /// ']' jumps to corresponding [
    const EIF: u8 = ']' as u8;
    /// '[' jumps to corresponding ] if 0 is set on position
    const JIF: u8 = '[' as u8;
    let mut indent_level = 0usize;
    id += 1;
    loop {
        let command = source[id];
        if command == JIF {
            indent_level += 1;
        } else if command == EIF && indent_level == 0 {
            return id - 1;
        } else if command == EIF && indent_level != 0 {
            indent_level -= 1;
        }
        id += 1;
    }
}

/*
INSTRUCTIONS:
> move pointer right
< move pointer left
+ increment position
- decrement position
. display value on position (ASCII number)
, gets a value from user and sets it on position
[ jumps to corresponding ] if 0 is set on position
] jumps to corresponding [
    other characters are comments

STATE:
i current pointer position
vec value table accesed by vec[i]
*/
