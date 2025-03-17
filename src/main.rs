/*
*/
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::env;
use std::collections::HashMap;

#[derive (Debug)]
struct ValueChange {
    time: u64,
    value: String,
}

#[derive (Debug)]
struct Var {
    var_type: String,
    size: u8,
    identifier: String,
    reference: String,
    changes: Vec<ValueChange>,
}

fn separate_line(cur: &mut String) -> Vec<String> {
    let mut line_split: Vec<String> = Vec::new();
    let mut substring: String = String::new();
    let mut break_point: bool;
    for s in cur.chars() {
        match s {
            ' ' => {
                break_point = true;
            },
            '\n' => {
                break_point = true;
            },
            '\t' => {
                break_point = true;
            },
            _ => {
                substring.push(s);
                break_point = false;
            },
        }

        if break_point {
            if substring.is_empty() == false {
                line_split.push(substring.clone());
                substring.clear();
            }
        }
    }

    /*if line_split.is_empty()== false {
        println!("{:?}", line_split);
    }*/

    return line_split;

}

fn handle_var(vec: Vec<String>) -> Var{
    let change: Vec<ValueChange> = Vec::new(); 

    let mut re: String;
    if vec[5].contains("$end") {
        re = vec[4].clone();
    } else {
        re = vec[4].clone();
        re.push_str(&vec[5]);
    }

    let var = Var {
        var_type: vec[1].clone(),
        size: vec[2].parse().unwrap(),
        identifier: vec[3].clone(),
        reference: re,
        changes: change,
    };

    return var;
}

fn main() {

    let mut current_time: u64 = 0;

    // Parse command line arguments for VCD file name
    let args = env::args();
    if args.len() != 2 {
        eprintln!("Must provide target VCD file");
        return;
    }
    let args: Vec<String> = args.collect();
    let input = args[1].clone();

    // Open VCD file into BufReader
    let vcd_file = match File::open(input) {
        Err(e) => panic!("{}", e),
        Ok(file) => file,
    };
    let mut reader: BufReader<File> = BufReader::new(vcd_file);

    // Buffer for dumping read content into
    let mut buf: String = String::new();

    // Hash Map of variable reference to variable identifier
    let mut vars_mapping: HashMap<String, u32> = HashMap::new();
    // Vector of all the variables
    let mut vars_values: Vec<Var> = Vec::new();

    let value = ['0', '1', 'x', 'X', 'z', 'Z'];
    let vector_value = ['b', 'B', 'r', 'R'];

    let mut past_dumpvars = false;

    // Loop until EOF is reached
    loop {
        let vectorized_line: Vec<String>;

        // Read the next line from the ReadBuffer
        let bytes_read: usize = match reader.read_line(&mut buf) {
            Err(why) => panic!("ERROR: Could not read from file: {}", why),
            Ok(size) => size,
        };

        // EOF reached
        if bytes_read == 0 {
            break;
        } 

        // Split the current line by spaces
        vectorized_line = separate_line(&mut buf);

        // If the split line was not an empty line
        if vectorized_line.len() > 0 {
            let first_string: String = vectorized_line[0].clone();
                
            if first_string.contains("$comment") {
                println!("{}", first_string);
            } else if first_string.contains("$date") {
                println!("{}", first_string);
            } else if first_string.contains("$enddefinitions") {
                println!("{}", first_string);
            } else if first_string.contains("$scope") {
                println!("{}", first_string);
            } else if first_string.contains("$timescale") {
                println!("{}", first_string);
            } else if first_string.contains("$upscope") {
                println!("{}", first_string);
            } else if first_string.contains("$var") {
                // Determine the index of the next Var and use later to quickly index
                // and update the value whenever it changes
                let vars_len = vars_mapping.len() as u32;
                vars_mapping.insert(vectorized_line[3].clone(), vars_len);
                vars_values.push(handle_var(vectorized_line));
            } else if first_string.contains("$version") {
                println!("{}", first_string);
            } else if first_string.contains("$dumpall") {
                println!("{}", first_string);
            } else if first_string.contains("$dumpoff") {
                println!("{}", first_string);
            } else if first_string.contains("$dumpon") {
                println!("{}", first_string);
            } else if first_string.contains("$dumpvars") {
                current_time = 0;
                past_dumpvars = true;
                println!("{}", first_string);
            } else if first_string.contains("$end") {
                println!("{}", first_string);
            } else {
                if past_dumpvars {
                    // Grab the very first character of the line
                    let first_char = match first_string.chars().nth(0) {
                        None => ' ',
                        Some(c) => c,
                    };

                    // Simulation time
                    if first_char == '#' {
                        current_time  = first_string[1..].parse().unwrap();

                    // Scalar value change
                    } else if value.contains(&first_char) {
                        let val_change = ValueChange {
                            time: current_time,
                            value: first_char.to_string(),
                        };
                        let identifier = &first_string[1..];
                        let cur_index = match vars_mapping.get(identifier) {
                            Some(&num) => num,
                            _ => panic!("Index not found for {}.", identifier),
                        };
                        let cur_index = cur_index as usize;

                        vars_values[cur_index].changes.push(val_change);

                    // Vector value change
                    } else if vector_value.contains(&first_char) {
                        // Second string in the line is the identifier
                        let identifier = &vectorized_line[1];

                        let val_change = ValueChange {
                            time: current_time,
                            value: first_string[1..].to_string(),
                        };

                        let cur_index = match vars_mapping.get(identifier) {
                            Some(&num) => num,
                            _ => panic!("Index not found for {}.", identifier),
                        };
                        let cur_index = cur_index as usize;

                        vars_values[cur_index].changes.push(val_change);
                    }
                }
            }
        }

        // Clear out the buffer 
        buf.clear();
    } // end loop


    // Determine which signals/registers do not change throughout the simulation
    let mut var_iter = vars_values.iter();

    println!("\nVariables that do not change:");
    while var_iter.len() > 0 {
        let current = match var_iter.next() {
            Some(var) => var,
            None => break,
        };

        if current.changes.len() <= 1 {
            if current.var_type != "parameter".to_string() {
                println!("Variable: {} Identifier: {} Change: {:?}", current.reference, current.identifier, current.changes);
                //println!("{:?}", current);
            }
        }
    }

    //println!("\nDebugging:\n");

    // Debugging
    //println!("{:?}", vars_mapping);
    println!("{:?}", vars_values);

    // Return
    return;
}
