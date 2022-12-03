use std::{fs::read_to_string, vec};
fn main() {
    let filename:&str = "src/input.txt";
    let data: String = read_to_string(filename).expect("Unable to read file!");
    let lines = data.lines();
    let mut current_elf_calories : i32 = 0;
    let mut top_three: Vec<i32> = vec![0, 0, 0];

    for line in lines {
        if let Ok(calories) = line.parse::<i32>() {
            current_elf_calories += calories;
        } else if line.trim().is_empty() {
            replace_smallest(&mut top_three, current_elf_calories);
            current_elf_calories = 0;
        } else {
            println!("Text file must only contain integers on each line!")
        }
    }
    replace_smallest(&mut top_three, current_elf_calories);

    println!("{}", top_three.iter().sum::<i32>())
}

fn get_smallest_index(vector: &Vec<i32>) -> usize {
    let mut cur_min: i32 = vector[0];
    let mut min_index:usize = 0;
    for (index, item) in vector.iter().enumerate() {
        if *item < cur_min {
            cur_min = *item;
            min_index = index;
        }
    }
    return min_index;
}

fn replace_smallest(vector: &mut Vec<i32>, number:i32) {
    let index:usize = get_smallest_index(vector);
    if number > vector[index] {
        vector[index] = number;
    }
}