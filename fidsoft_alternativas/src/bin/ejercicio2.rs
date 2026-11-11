use fidsoft_alternativas::utils;

fn main () {
    let mut buffer = String::new();
    let number: i32 = utils::read_i32("Introduce first number", &mut buffer);
    let even_or_odd: bool = number % 2 == 0;
    let message: &str = match even_or_odd {
      true => "the number is even",
      false => "the number is odd"
    };
    println!("{}", message);
}