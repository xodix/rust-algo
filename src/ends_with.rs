pub fn ends(word: &str, ending: &str) -> bool {
  for i in 0..(&ending).len() {
    if ending.chars().nth_back(i) != word.chars().nth_back(i) {
      return false;
    }
  }
  return true;
}
