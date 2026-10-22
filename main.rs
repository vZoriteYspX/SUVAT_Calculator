use std::collections::HashMap;
use std::process::ExitCode;
mod na_inputs;
mod na_uam;

fn main() -> ExitCode {
    loop {
        let var_out: Vec<String> = na_inputs::input_variables();
        let val_out: HashMap<String, f64> = na_inputs::input_values(&var_out);
        let unknwn_out: String = na_inputs::input_unknwn_var(&var_out);
        let result_val: HashMap<String, f64> = na_uam::uam(&val_out, unknwn_out);
        
        println!();
        println!("-------------------------");
        println!("{:?}", result_val);

        loop {
            match na_inputs::input_continue() {
                1 => return ExitCode::SUCCESS,
                2 => break,
                _ => continue,
            }
        }
    }
}