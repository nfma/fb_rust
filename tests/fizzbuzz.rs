extern crate fizzbuzz;

#[test]
fn zero_is_fizzbuzz() {
  assert_eq!("fizzbuzz", fizzbuzz::fizzbuzz::run(0, 0));
}

#[test]
fn non_multiple_of_3_or_5_returns_the_number() {
  assert_eq!("1", fizzbuzz::fizzbuzz::run(1, 1));
}

#[test]
fn multiple_of_3_is_fizz() {
  assert_eq!("fizz", fizzbuzz::fizzbuzz::run(3, 3));
}

#[test]
fn multiple_of_5_is_buzz() {
  assert_eq!("buzz", fizzbuzz::fizzbuzz::run(5, 5));
}

#[test]
fn multiple_of_15_is_fizzbuzz() {
  assert_eq!("fizzbuzz", fizzbuzz::fizzbuzz::run(15, 15));
}

#[test]
fn the_first_20_numbers() {
  assert_eq!("1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz 16 17 fizz 19 buzz", fizzbuzz::fizzbuzz::run(1, 20));
}
