use std::io::{self, Write};
use std::collections::HashMap;

pub fn input_variables() -> Vec<String> {
    let mut variables: Vec<String> = vec![];

    println!();
    println!("-------------------------");
    println!("Please enter your SUVAT variables one by one. Input \"done\" once you have finished inputting your variables.");
    println!("SUVAT Variables:");
    println!("\"displacement\" (Displacement)");
    println!("\"initialv\" (Initial Velocity");
    println!("\"finalv\" (Final Velocity");
    println!("\"acceleration\" (Acceleration)");
    println!("\"time\" (Time)");

    loop {
        let mut var_in: String = String::new();

        println!("-------------------------");
        print!("Input here: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut var_in)
            .expect("Failed to read line.");
        let var_trim: String = var_in.trim().to_lowercase().to_string();

        let var_name = match var_trim.as_str() {
            "displacement" => "Displacement",
            "initialv" => "Initial Velocity",
            "finalv" => "Final Velocity",
            "acceleration" => "Acceleration",
            "time" => "Time",
            "done" => {
                if (3..=4).contains(&variables.len()) {
                    println!("Proceeding to the next step.");
                    return variables;
                } else if variables.len() >= 5 {
                    println!("-------------------------");
                    println!("Resetting, too many variables! Please try again.");
                    variables.clear();
                    continue;
                } else {
                    println!("Please input 3-4 known variables.");
                    continue;
                }
            },

            _ => {
                println!("Invalid input. Please try again.");
                continue;
            },
        };

        if variables.contains(&var_trim) {
            println!("Your \"known\" variable has already been inputted. Please try again.");
            continue;
        }

        println!("-------------------------");
        println!("{} added!", var_name);
        variables.push(var_trim);
        continue;
    }
}


pub fn input_unknwn_var(knwn_vars: &Vec<String>) -> String {
    loop {
        let mut unknwn_var_in: String = String::new();

        println!("-------------------------");
        println!("Which SUVAT variable are you looking for?");
        print!("Input here: ");
        io::stdout()
            .flush()
            .expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut unknwn_var_in)
            .expect("Failed to read line.");
        let unknwn_var_trim: String = unknwn_var_in.trim().to_lowercase().to_string();

        if knwn_vars.contains(&unknwn_var_trim) {
            println!("Your \"unknown\" variable has already been inputted. Please try again.");
            continue;

        } else {
            match unknwn_var_trim.as_str() {
                "displacement" | "initialv" | "finalv" | "acceleration" | "time" => {
                    println!("-------------------------");
                    println!("Unknown variable added.");
                    println!("Proceeding to the next step.");
                    return unknwn_var_trim;
                }
                _ => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }
}


pub fn input_values(variables: &Vec<String>) -> HashMap<String, f64> {
    let mut knwn_vals: HashMap<String, f64> = HashMap::new();

    for suvat_var in variables {
        loop {
            println!();
            println!("-------------------------");
            println!("Enter the value for {} in its respective SI unit(s).", suvat_var);
            let mut variable: String = String::new();
            io::stdin()
                .read_line(&mut variable)
                .expect("Failed to read line.");
            let trim_var = variable.trim();
            
            if trim_var.is_empty() {
                println!("-------------------------");
                println!("Input cannot be empty. Please try again.");
                continue;
            }

            let var_double: f64 = 
                match trim_var.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("-------------------------");
                        println!("Please input a valid number.");
                        continue;
                    }
                };

            knwn_vals.insert(suvat_var.to_string(), var_double);
            break;
        }
    }

    return knwn_vals;
}

pub fn input_continue() -> i32 {
    println!("-------------------------");
    println!("Continue? [y / n]");
    let mut response: String = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");
    let trim_response = response.trim();
    let response_char: char =
        match trim_response.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("-------------------------");
                println!("Please input \"y\" or \"n\".");
                return 0;
            }
        };

    if response_char == 'y' {
        println!("-------------------------");
        println!("New Calculation");
        println!("");
        return 2;

    } else if response_char == 'n' {
        println!("-------------------------");
        println!("Thank you for using this SUVAT calculator!");
        return 1;

    } else {
        println!("-------------------------");
        println!("Please input \"y\" or \"n\".");
        return 0;
    }
}
