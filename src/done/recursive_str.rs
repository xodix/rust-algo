recursive_rev_str(&mut vec![
    'H', 'e', 'l', 'l', 'o', ' ', 'M', 'a', 't', 'e', '!',
]);
fn recursive_rev_str(s: &mut Vec<char>) -> () {
    if s.len() == 0 {
        return;
    }
    println!("{}", s[s.len() - 1]);
    recursive_rev_str(&mut s.get(0..(s.len() - 1)).unwrap().to_vec());
}
