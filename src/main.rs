use std::fs;

//use _03::check_if_symbol;
use _03::{sum_up_schematic,sum_up_gears};

fn main() {
    let example = fs::read_to_string("example.txt").expect("example.txt cannot be opened");

    //println!("{}",example);
    println!("Going into sum_up_schematic_function");

    let sum = sum_up_schematic(&example);
    println!("sum = {}",sum);
        

    let sum2 = sum_up_gears(&example);
    println!("gear sum = {}",sum2);
}
