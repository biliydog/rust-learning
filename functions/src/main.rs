fn main() {
  println!("Hello, world!");
  anther_function(3);
  test_expression()
}

fn anther_function(x: i32) {
  println!("the paramter is {}", x)
}

fn test_expression() {
  let y = {
    let x = 3;
    x + 1
  };
  println!("y is {}", y)
}
