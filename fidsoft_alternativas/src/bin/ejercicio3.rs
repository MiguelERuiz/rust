use fidsoft_alternativas::utils;

fn main () {
    let mut buffer = String::new();
    let number: i32 = utils::read_i32("Introduce number", &mut buffer);
    let positive_or_negative: bool = number >= 0;
    let message: &str = match positive_or_negative {
      true => "the number is positive",
      false => "the number is negative"
    };
    println!("{}", message);
}