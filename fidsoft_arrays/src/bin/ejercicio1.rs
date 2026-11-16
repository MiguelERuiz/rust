use fidsoft_arrays::utils;

fn main() {
  let array = [1,2,3,4,5];
  let mut buffer = String::new();
  let number = utils::read_i32("Introduce an integer number", &mut buffer);
  let mut found = false;
  for i in array {
    if !found && i == number {
      found = true;
    }
  }

  println!("The number {number} is in the array? {found}");
}