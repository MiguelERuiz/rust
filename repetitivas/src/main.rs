mod ejercicio_1;
mod ejercicio_2;
mod ejercicio_3;
mod ejercicio_4;
mod ejercicio_5;

use ejercicio_1::ejercicio1;
use ejercicio_2::ejercicio2;
use ejercicio_3::ejercicio3;
use ejercicio_4::ejercicio4;
use ejercicio_5::ejercicio5;
use repetitivas::utils;

fn main() {
    loop {
        println!("\n=== MENÚ ===");
        println!("1. Números del 100 al 50 de 5 en 5");
        println!("2. Tablero con dado");
        println!("3. Tabla de multiplicar");
        println!("4. Autobús");
        println!("5. Adivina el número");
        println!("0. Salir");
        match utils::leer_numero("Elige una opción: ") {
            1 => ejercicio1(),
            2 => ejercicio2(),
            3 => ejercicio3(),
            4 => ejercicio4(),
            5 => ejercicio5(),
            0 => break,
            _ => println!("Opción no válida."),
        }
    }
}
