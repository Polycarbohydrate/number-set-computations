use std::collections::HashMap;
use std::io;
use ordered_float::OrderedFloat;
use crate::input::mode;

pub fn mode_calculation()  {
    let mut dataset: HashMap<OrderedFloat<f64>, u64> = HashMap::new();
    println!("=============================================================================");
    println!("Enter your set of numbers here.");
    println!("Enter each number on a new line. Press enter to move to another line.");
    println!("Once you have entered all numbers, type 'exit' in a new line and press enter.");
    println!("=============================================================================");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "exit" => {
                println!("========");
                println!("Exiting!");
                break;
            }
            value => match value.parse::<f64>() {
                Ok(_n) => {
                    let f = OrderedFloat(input.to_string().replace("\r\n", "").parse::<f64>().unwrap());
                    if dataset.contains_key(&f) {
                        let h = dataset.get_mut(&f).unwrap();
                        *h += 1;
                    } else {
                        dataset.insert(f, 1);
                    }
                }
                Err(_) => {
                    println!("======================");
                    println!("Please enter a number!");
                    println!("======================");
                }
            }
        }
    }
    if let Some((key,_max_value)) = dataset.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) {
        println!("===============");
        println!("The mode is {}.", key);
        mode();
    } else {
        println!("=================");
        println!("The map is empty.");
    }
}
