use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        // Crear una instancia del generador de números aleatorios
        let mut rng = rand::thread_rng();

        // Crear un vector para almacenar la combinación de números
        let mut combination: Vec<u32> = Vec::new();

        // Generar 5 números aleatorios en el rango de 1 a 23 
        for _ in 0..5 {
            let random_number = rng.gen_range(1..24);
            combination.push(random_number);
        }

        // Imprimir la combinación generada
        println!("Combinación generada: {:?}", combination);

        // Dormir durante 24 horas antes de la próxima ejecución
        thread::sleep(Duration::from_secs(86400));
    }
}


