use crate::utils;

pub fn ejercicio4() {
    let mut asientos = ["0", "1", "0", "1", "1", "1", "0", "0", "1", "0"];
    loop {
        let numero_asiento = utils::leer_numero("Selecciona un numero de asiento entre 1 y 10: ");
        if numero_asiento < 1 || numero_asiento > 10 {
            println!("Numero asiento {numero_asiento} invalido");
        } else {
            let numero_asiento_sanitize = (numero_asiento - 1) as usize; // this is done since arrays index are from 0 to 9
            if asientos[numero_asiento_sanitize] == "1" {
                println!("Asiento con número {numero_asiento} está ocupado");
            } else {
                asientos[numero_asiento_sanitize] = "1";
                println!("Reservado asiento numero {numero_asiento}!!");
            }
        }
        let operacion = utils::leer("Desea hacer otra operacion??(S/N): ");
        match operacion.to_uppercase().as_str() {
            "N" => break,
            _ => continue,
        }
    }
}
