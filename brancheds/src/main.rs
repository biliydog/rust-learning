fn main() {
  let mut s = String::from("hello world");
  let t = &s;
  let c = &s[8..20];
  let res = first_words(t);
  s.clear();
  println!("res is {},c is {}", res, c)
}

fn first_words(s: &String) -> &str {
  let bts = s.as_bytes();
  for (i, &item) in bts.iter().enumerate() {
    if item == b' ' {
      return &s[..i];
    }
  }
  &s[..]
}
