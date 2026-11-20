use crate::utils;

pub fn ejercicio3() {
    let numero = utils::leer_numero("Introduce un número: ");
    println!("\n");
    println!("========= Tabla del {numero} ==========");
    println!("================================");
    for n in 1..11 {
        let resultado = n * numero;
        println!("\t  {numero} * {n} = {resultado}")
    }
    println!("================================");
}
