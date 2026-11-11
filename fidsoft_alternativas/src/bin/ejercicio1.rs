use fidsoft_alternativas::utils;

fn main() {
  let mut buffer = String::new();
  let first_number: i32 = utils::read_i32("Introduce first number", &mut buffer);
  let second_number: i32 = utils::read_i32("Introduce second number", &mut buffer);
  let operator: &str = utils::read_str("Introduce operator", &mut buffer);
  let result: i32 = match operator {
      "+" => first_number + second_number,
      "-" => first_number - second_number,
      "*" => first_number * second_number,
      "/" => first_number / second_number,
      _   => panic!("Unsupported operation"),
  };
  println!("The result of the operation {} between {} and {} is {}", operator, first_number, second_number,  result);
}