pub fn ejercicio2() {
    let posiciones_tablero: i8 = 25;
    let mut casillas_recorridas: i8 = 0;
    while casillas_recorridas <= posiciones_tablero {
        println!("Tiro el dado!!");
        let tirada = rand::random_range(1..=6);
        println!("Ha salido un {tirada}");
        casillas_recorridas += tirada;
        println!("Avanzo {casillas_recorridas} casillas");
    }
    println!("Fin del juego")
}
