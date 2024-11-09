use std::io;
use crate::computation::{mean, median, mode, range};

pub fn run() {
    loop {
        println!("===========================================================================");
        println!("Select the calculation type.");
        println!("1. Mean (Average of a given set.)");
        println!("2. Median (Middle number in a given set.)");
        println!("3. Mode (Most common number in a given set.)");
        println!("4. Range (Difference between greatest and least integers.)");
        println!("Press either '1' for Mean, '2' for Median, '3' for Mode, and '4' for Range.");
        println!("===========================================================================");

        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read line");
        let selection: u8 = match selection.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("=============================================================");
                println!("Please enter a positive integer that is either 1, 2, 3, or 4.");
                continue
            }
        };
        if selection == 3   {
            // Mode is different because other ones can use the vector
            // starting in line 41 for the data but mode uses a
            // HashMap
            mode::mode_calculation();
            break
        }
        if selection > 4    {
            println!("=============================================================");
            println!("Please enter a positive integer that is either 1, 2, 3, or 4.");
            run();
        }
        else if selection < 1  {
            println!("=============================================================");
            println!("Please enter a positive integer that is either 1, 2, 3, or 4.");
            run();
        }
        let data = get_number_set();
        match selection {
            1 =>    {
                mean(data);
                break
            }
            2 =>    {
                median(data);
                break
            }
            3 =>    {
                mode();
                break
            }
            4 =>    {
                range(data);
                break
            }
            _ =>    {
                println!("================================");
                println!("Please enter only 1, 2, 3, or 4!");
                run();
            }
        }
    }
}


pub fn get_number_set() -> Vec<f64> {
    println!("=============================================================================");
    println!("Enter your set of numbers here.");
    println!("Enter each number on a new line. Press enter to move to another line.");
    println!("Once you have entered all numbers, type 'exit' in a new line and press enter.");
    println!("=============================================================================");

    let mut data = Vec::new();
    loop {
        let mut dataset = String::new();
        io::stdin().read_line(&mut dataset).expect("Failed to read line");
        match dataset.trim() {
            "exit" => {
                println!("========");
                println!("Exiting!");
                println!("========");
                break;
            }
            _ =>    {
                match dataset.trim().parse::<f64>() {
                    Ok(n) => data.push(n),
                    Err(_) => {
                        println!("========================");
                        println!("Please enter an integer.");
                        println!("========================");
                        continue
                    }
                }
            }
        }
    }
    data
}

pub fn mean(data: Vec<f64>)    {
    mean::mean_main(data);
    loop {
        println!("===============================================");
        println!("Would you like to make another calculation? y/n");
        println!("===============================================");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read line");
        if again.to_lowercase().trim() == "y" {
            run();
            break
        }   else {
                break
            }
    }
}

pub fn median(data: Vec<f64>)    {
    median::median_main(data);
    loop {
        println!("===============================================");
        println!("Would you like to make another calculation? y/n");
        println!("===============================================");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read line");
        if again.to_lowercase().trim() == "y" {
            run();
            break
        }   else {
            break
        }
    }
}

pub fn mode()    {
    loop {
        println!("===============================================");
        println!("Would you like to make another calculation? y/n");
        println!("===============================================");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read line");
        if again.to_lowercase().trim() == "y" {
            run();
            break
        }   else {
            break
        }
    }
}

pub fn range(data: Vec<f64>)    {
    range::range_main(data);
    loop {
        println!("===============================================");
        println!("Would you like to make another calculation? y/n");
        println!("===============================================");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read line");
        if again.to_lowercase() == "y" {
            run();
            break
        }   else {
            break
        }
    }
}