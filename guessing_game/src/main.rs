use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Lets play guessing game");
  let secrect_number = rand::thread_rng().gen_range(1..101);
  loop {
    println!("please input your guess");
    println!("the secret number is:{}", secrect_number);
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Faild to read Line");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    println!("you guessed:{}", guess);
    match guess.cmp(&secrect_number) {
      Ordering::Less => println!("too small"),
      Ordering::Equal => {
        println!("you win");
        break;
      }
      Ordering::Greater => println!("too big"),
    }
  }
}
