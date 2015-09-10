pub fn run(start: i32, end: i32) -> String {
  (start..end).fold(calculate(start), |result, num| result + " " + &calculate(num+1))
}

fn calculate(num: i32) -> String {
  if num % 15 == 0 {
    String::from("fizzbuzz")
  } else if num % 3 == 0 {
    String::from("fizz")
  } else if num % 5 == 0 {
    String::from("buzz")
  } else {
    num.to_string()
  }
}
