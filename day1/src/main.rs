use std::{fs::read_to_string};
fn main() {
    let filename:&str = "src/input.txt";
    let data: String = read_to_string(filename).expect("Unable to read file!");
    let lines = data.lines();
    let mut current_elf_calories : i32 = 0;
    let mut result: i32 = 0;

    for line in lines {
        if let Ok(calories) = line.parse::<i32>() {
            current_elf_calories += calories;
            if current_elf_calories > result {
                result = current_elf_calories;
            }
        } else if line.trim().is_empty() {
            current_elf_calories = 0;
        } else {
            println!("Text file must only contain integers on each line!")
        }
    }

    println!("{}", result)
}

