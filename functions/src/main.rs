use std::io;

fn main() {
    println!("Hello, world!");
    // sum_and_concatenate(10, 20, 'a', 'b');
    // eval_function();
    // let result: i32 = new_expression(5);
    // println!("Result is: {}", result);
    // if_else_control();
    even_loop_control();
}

fn even_loop_control() {
    let input: String = get_user_input();
    let number: i32 = string_to_integer(input);

    let mut count: i32 = 0;
    loop {
        if count >= number {
            println!("Limite alcanzado");
            break;
        }
        if count % 2 == 0 {
            count += 1;
            continue;
        }
        println!("numero impar {}", count);
        count += 1;
    }
}

// fn if_else_control() {
//     let entrada: String = get_user_input();
//     let numbero: i32 = string_to_integer(entrada);

//     if numbero < 0 {
//         println!("Negative value");
//     } else if numbero > 0 {
//         println!("Positive value");
//     } else {
//         println!("Zero value");
//     }
// }

fn get_user_input() -> String {
    println!("Por favor introduzca un valor:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn string_to_integer(input: String) -> i32 {
    let number: i32 = input.parse().unwrap();
    number
}

// fn sum_and_concatenate(n1: u8, n2: u8, char1: char, char2: char) {
//     let suma: u8 = n1 + n2;
//     println!("The sum number is {}", suma);

//     println!("The union of characters is {}{}", char1, char2);
// }

// fn eval_function() {
//     let evaluable = {
//         let number = 3;
//         number + 2
//     };
//     println!("The value of variable is {evaluable}");
// }

// fn new_expression(x: i32) -> i32 {
//     x
// }
