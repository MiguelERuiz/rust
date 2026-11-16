use rand;

fn main() {

  // Generates 300 random integers from -20 to 20 (inclusive)
  let vec: Vec<i32> = (0..300)
      .map(|_| rand::random_range(-20..=20))
      .collect();

  let mut count_positives: i32 = 0;
  let mut count_negatives: i32 = 0;
  let mut count_zeros: i32 = 0;

  for i in vec {
    if i > 0 {
      count_positives += 1;
    } else if i < 0 {
      count_negatives += 1;
    } else {
      count_zeros += 1;
    }
  }

  println!("There are {count_positives} positive numbers, {count_negatives} negative numbers and {count_zeros} zeros");
}