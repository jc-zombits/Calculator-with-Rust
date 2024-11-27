use std::io;

fn main() {
    println!("###### CALCULATOR ######");

    // Pedir al usuario que seleccione una operación
    println!("Select an operation:\n1. Addition\n2. Subtraction\n3. Multiplication\n4. Division");

    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input!");

    let operation: i32 = operation.trim().parse().expect("Please enter a valid number!");

    // Pedir al usuario los dos números si la operación es válida
    if operation >= 1 && operation <= 4 {
        println!("Enter the first number: ");
        let mut num1 = String::new();

        io::stdin().read_line(&mut num1).expect("Failed to read input!");

        let num1: f64 = num1.trim().parse().expect("Please enter a valid number!");

        println!("Enter the second number: ");
        let mut num2 = String::new();

        io::stdin().read_line(&mut num2).expect("Failed to read input!");

        let num2: f64 = num2.trim().parse().expect("Please enter a valid number!");

        // Realizar la operación seleccionada
        let result = match operation {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Division by zero is not allowed!");
                    return;
                }
            }
            _ => unreachable!(), // Esto nunca debería ocurrir
        };

        println!("The result is: {}", result);
    } else {
        println!("Invalid operation. Please select a number between 1 and 4.");
    }
}
