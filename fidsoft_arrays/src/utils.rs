use std::io;

pub fn read_f32(message: &str, buffer: &mut String) -> f32 {
  buffer.clear();
  println!("{message}");

  io::stdin()
    .read_line(buffer)
    .expect("Read error");

  buffer.trim()
        .parse()
        .expect("Not a number")
}

pub fn read_i32(message: &str, buffer: &mut String) -> i32 {
  buffer.clear();
  println!("{message}");

  io::stdin()
    .read_line(buffer)
    .expect("Read error");

  buffer.trim()
        .parse()
        .expect("Not a number")
}

pub fn read_str<'a>(message: &'a str, buffer: &'a mut String) -> &'a str {
  buffer.clear();
  println!("{message}");

  io::stdin()
    .read_line(buffer)
    .expect("Read error");

  buffer.trim()
}