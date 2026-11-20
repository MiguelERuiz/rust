use std::io;
use std::io::Write;

/// Lee un número entero, repitiendo hasta que sea válido
pub fn leer_numero(mensaje: &str) -> i32 {
    loop {
        match leer(mensaje).parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Eso no es un número válido."),
        }
    }
}

/// Muestra un mensaje y lee una línea del teclado
pub fn leer(mensaje: &str) -> String {
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    let mut teclado = String::new();
    io::stdin().read_line(&mut teclado).unwrap();
    teclado.trim().to_string()
}
