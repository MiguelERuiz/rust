fn main() {
    let array: [i8] = [10,2,3,4,5];
    let mut maximo = 0;
    let mut suma = 
    for i in array {
        if i > maximo {
            maximo = i
        }
    }
    println!("{}", maximo);
//    let minimo = array.min();
//    let suma = array.sum();
    println!("array: {:?}", array);
    println!("maximo: {}", maximo);
    println!("minimo: {}", minimo);
    println!("suma: {}", suma);
}
