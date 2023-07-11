use std::io;
use pause_console::*;

//Function that takes the user input and convert it to a vector
fn input_str_2_vec(n: String) -> Vec<String> {
    let num_user_in: Vec<String> = n.split_whitespace().map(str::to_string).collect();
    return num_user_in
}

fn main() {
    loop {
    //print "Enter and Expression" and get user input
    let mut user_in = String::new();
    println!("\nEnter an Expression: ");
    io::stdin().read_line(&mut user_in).expect("Not Valid");

    print!("{}", user_in);
    
    let final_vec: Vec<String> = input_str_2_vec(user_in);
    let num1 = final_vec[0].trim().parse::<f32>().unwrap();
    let num2 = final_vec[2].parse::<f32>().unwrap();
    let operator = &final_vec[1];
    let mut fin: f32 = 0.0;
    
    //Mathcing operators to operations
    match operator.as_str() {
        "+" => fin = num1 + num2,
        "-" => fin = num1 - num2,
        "x" => fin = num1 * num2,
        "/" => fin = num1 / num2,
        _ => println!("Only addition, subtraction, multiplication and division are allowed"),
    }

    println!("Result:{}", fin);
    pause_console!("Press Enter for new expression or 'Ctrl + C' to quit...");
    std::process::Command::new("clear").status().unwrap();
    }  
}
