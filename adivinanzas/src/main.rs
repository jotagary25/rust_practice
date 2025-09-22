use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("El número secreto es: {secret_number}");

    loop {
        println!("Ingresa tu respuesta:");
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Fallo al leer la linea!");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&secret_number) {
            Ordering::Less => println!("Tu número es menor"),
            Ordering::Greater => println!("Tu número es mayor"),
            Ordering::Equal => {
                println!("¡Has ganado!");
                break;
            }
        }
    }
}
