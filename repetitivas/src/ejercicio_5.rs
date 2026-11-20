use crate::utils;
use std::cmp::Ordering;

pub fn ejercicio5() {
    println!("Adivina el número que estoy pensando entre 1 y 50");
    let numero_secreto = rand::random_range(1..=50);
    loop {
        let numero = utils::leer_numero("Introduce tu número: ");
        match numero.cmp(&numero_secreto) {
            Ordering::Less => println!("Es muy pequeño!"),
            Ordering::Greater => println!("Es muy grande!"),
            Ordering::Equal => {
                println!("Has acertado!");
                break;
            }
        }
    }
}
