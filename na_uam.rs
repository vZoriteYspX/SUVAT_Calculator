use std::collections::HashMap;

pub fn uam(knwn_vars: &HashMap<String, f64>, unknwn_var: String) -> HashMap<String, f64> {

    let error: &str = "error";
    let mut formula: i32 = 0;
    let mut result: HashMap<String, f64> = HashMap::new();

    match unknwn_var.as_str() {
        "displacement" => {
            formula = 1;
        } "initialv" => {
            formula = 2;
        } "finalv" => {
            formula = 3;
        } "acceleration" => {
            formula = 4;
        } "time" => {
            formula = 5;
        } _ => {
            println!("An error has occured. Please try again.");
        }
    }

    match formula {
        1 => {
            if !knwn_vars.contains_key("initialv") {
                //  s = vt - (at^2)/2
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let s_val: f64 = (v_val * t_val) - ((a_val * t_val.powf(2.0)) / 2.0);
                result.insert(unknwn_var, s_val);
                return result;

            } else if !knwn_vars.contains_key("finalv") {
                //  s = ut + (at^2)/2
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let s_val: f64 = (u_val * t_val) + ((a_val * t_val.powf(2.0)) / 2.0);
                result.insert(unknwn_var, s_val);
                return result;


            } else if !knwn_vars.contains_key("acceleration") {
                //  s = (v + u)t / 2
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let s_val: f64 = (v_val + u_val) * (t_val / 2.0);
                result.insert(unknwn_var, s_val);
                return result;


            } else {
                //  s = (v^2 - u^2) / 2a
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let s_val: f64 = (v_val.powf(2.0) - u_val.powf(2.0)) / (2.0 * a_val);
                result.insert(unknwn_var, s_val);
                return result;

            }
        }
        
        2 => {
            if !knwn_vars.contains_key("displacement") {
                //  u = v - at
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let u_val: f64 = v_val - (a_val * t_val);
                result.insert(unknwn_var, u_val);
                return result;

            } else if !knwn_vars.contains_key("finalv") {
                //  u = s/t - (at)/2
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let u_val: f64 = s_val / t_val - a_val * t_val / 2.0;
                result.insert(unknwn_var, u_val);
                return result;

            } else if !knwn_vars.contains_key("acceleration") {
                //  u = 2s/t - v
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let u_val: f64 = 2.0 * s_val / t_val - v_val;
                result.insert(unknwn_var, u_val);
                return result;

            } else {
                //  u = (v^2 - 2as)^(1/2)
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let square: f64 = v_val.powf(2.0) - (2.0 * a_val * s_val);
                if square < 0.0 {
                    let u_val: f64 = square.abs().sqrt() * -1.0;
                    result.insert(unknwn_var, u_val);
                    return result;
                } else {
                    let u_val: f64 = square.sqrt();
                    result.insert(unknwn_var, u_val);
                    return result;
                }
            }
        }

        3 => {
            if !knwn_vars.contains_key("displacement") {
                //  v = u + at
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let v_val: f64 = u_val + (a_val * t_val);
                result.insert(unknwn_var, v_val);
                return result;

            } else if !knwn_vars.contains_key("initialv") {
                //  v = s/t + (at)/2
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let v_val: f64 = (s_val / t_val) + (a_val * t_val / 2.0);
                result.insert(unknwn_var, v_val);
                return result;

            } else if !knwn_vars.contains_key("acceleration") {
                //  v = 2s/t - u
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let v_val: f64 = (2.0 * s_val / t_val) - u_val;
                result.insert(unknwn_var, v_val);
                return result;

            } else {
                //  v = (u^2 + 2as)^(1/2)
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let square: f64 = u_val.powf(2.0) + (2.0 * a_val * s_val);
                if square < 0.0 {
                    let v_val: f64 = square.abs().sqrt() * -1.0;
                    result.insert(unknwn_var, v_val);
                    return result;
                } else {
                    let v_val: f64 = square.sqrt();
                    result.insert(unknwn_var, v_val);
                    return result;
                }
            }
        }

        4 => {
            if !knwn_vars.contains_key("displacement") {
                //  a = (v - u)/t
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let a_val: f64 = (v_val - u_val) / t_val;
                result.insert(unknwn_var, a_val);
                return result;

            } else if !knwn_vars.contains_key("initialv") {
                //  a = 2(s - vt)(-t^-2)
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let a_val: f64 = 2.0 * (s_val - (v_val * t_val)) * (1.0 / t_val.powf(2.0) * -1.0);
                result.insert(unknwn_var, a_val);
                return result;

            } else if !knwn_vars.contains_key("finalv") {
                //  a = 2(s - ut)(t^-2)
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let t_val: f64 = *knwn_vars.get("time").unwrap();
                let a_val: f64 = 2.0 * (s_val - (u_val * t_val)) * (1.0 / t_val.powf(2.0));
                result.insert(unknwn_var, a_val);
                return result;

            } else {
                //  a = (v^2 - u^2) / 2s
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let a_val: f64 = (v_val.powf(2.0) - u_val.powf(2.0)) / (2.0 * s_val);
                result.insert(unknwn_var, a_val);
                return result;
            }
        }

        5 => {
            if !knwn_vars.contains_key("displacement") {
                //  t = (v - u)/a
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let t_val: f64 = (v_val - u_val) / a_val;
                result.insert(unknwn_var, t_val);
                return result;

            } else if !knwn_vars.contains_key("initialv") {
                //  t = (v + (v^2 - 2as)^(1/2))/a
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let square: f64 = v_val.powf(2.0) - (2.0 * a_val * s_val);
                if square < 0.0 {
                    let t_val: f64 = v_val + (square.abs().sqrt() * -1.0 / a_val);
                    result.insert(unknwn_var, t_val);
                    return result;
                } else {
                    let t_val: f64 = v_val + (square.sqrt() / a_val);
                    result.insert(unknwn_var, t_val);
                    return result;
                }

            } else if !knwn_vars.contains_key("finalv") {
                //  t = (-u + (u^2 + 2as)^(1/2))/a
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let a_val: f64 = *knwn_vars.get("acceleration").unwrap();
                let binomial: f64 = u_val.powf(2.0) + (2.0 * a_val * s_val);
                if binomial < 0.0 {
                    let t_val: f64 = (u_val * -1.0) + (binomial.abs().sqrt() * -1.0  / a_val);
                    result.insert(unknwn_var, t_val);
                    return result;
                } else {
                    let t_val: f64 = (u_val * -1.0) + (binomial.sqrt() / a_val);
                    result.insert(unknwn_var, t_val);
                    return result;
                }

            } else {
                //  t = 2s/(v + u)
                let s_val: f64 = *knwn_vars.get("displacement").unwrap();
                let u_val: f64 = *knwn_vars.get("initialv").unwrap();
                let v_val: f64 = *knwn_vars.get("finalv").unwrap();
                let t_val: f64 = (2.0 * s_val) / (v_val + u_val);
                result.insert(unknwn_var, t_val);
                return result;
            }
        }

        _ => {
            println!("An error has occured. Please try again.");
            result.insert(error.to_string(), 0.0);
            return result;
        }
    }
}