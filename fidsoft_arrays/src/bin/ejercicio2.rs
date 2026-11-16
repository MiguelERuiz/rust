use fidsoft_arrays::utils;

#[allow(unused_assignments)]
fn main() {
  let mut seats = ["0", "1", "0", "1", "1", "1", "0", "0", "1", "0"];
  let mut buffer = String::new();
  let seat_number = utils::read_i32("Introduce an integer number between 1 and 10", &mut buffer);

  if seat_number < 1 || seat_number > 10 {
    panic!("Invalid seat_number");
  } else {
    let seat_number_sanitize = (seat_number - 1) as usize; // this is done since arrays index are from 0 to 9
    if seats[seat_number_sanitize] == "1" {
      println!("Seat with number {seat_number} is busy");
    } else {
    seats[seat_number_sanitize] = "1";
    println!("Reserved Happy traveling!");
    }
  }
}