// shadowing en Rust
// Enmascarar una variable no funciona igual que hacer mutable dicha variable
// Cuando enmascaramos en realidad estamos creando una nueva variable con el mismo nombre
// Sin embargo si hacemos una variable mutable, estamos cambiando su valor en memoria.
// fn main() {
//     let number = 45;
//     println!("The value of x is: {number}");

//     {
//         let number = number * 2;
//         println!("The value of x is: {number}");
//     }

//     let number = number * 1;
//     println!("The value of number is: {number}");
// }

fn main() {
    let suma = 6 + 10;
    println!("la suma es {suma}");

    let diferencia = 54.21 - 12.34;
    println!("la diferencia es {diferencia}");

    let producto = 5 * 7;
    println!("el producto es {producto}");

    let division = 120 / 7;
    println!("la division es {division}");

    let resto = 120 % 7;
    println!("el resto es {resto}");
}
