use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::usize;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let null = &'.';
    let mut line_length:i64 = 0;
    let mut gear_parts:HashMap<usize, Vec<i64>> = HashMap::new();
    let entries:Vec<char> = reader.lines()
        .map(|line| line.unwrap())
        .inspect(|line| line_length = line.len().try_into().unwrap())
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut i:i64 = 0;
    while i <= entries.len().try_into().unwrap_or(0){ 
        if !entries.get(usize::try_from(i).unwrap()).unwrap_or(null).is_numeric(){
            i = i + 1;
            continue;
        }
        let mut j:i64 = 0;
        let mut gear_indices:HashSet<usize> = HashSet::new();
        if let Some(gear_index) = check_neighbors(&entries, i, line_length){
            gear_indices.insert(gear_index);
        }
        while entries.get(usize::try_from(i+j).unwrap()).unwrap_or(null).is_numeric()
           &&  ((i+j) as f64 / line_length as f64).fract() != 0.0{
            if let Some(gear_index) = check_neighbors(&entries, i+j, line_length){
                gear_indices.insert(gear_index);
            }
            println!("{}", (i+j)%line_length != 0);
            j = j + 1;
        }
        let parsed_num = &entries[i as usize..(i+j) as usize].iter().collect::<String>();//.parse::<i64>().unwrap();
        println!("{}", parsed_num);
//        println!("\n{}", parsed_num);
       // for gear_index in gear_indices.iter(){
       //     gear_parts.entry(*gear_index).or_insert_with(Vec::new).push(*parsed_num);
        //}
        i = i + 1 + j;
    }
    let gear_part_ratio_product_sum = 
    gear_parts
        .values()
        .into_iter()
        .inspect(|lorem| println!("{:?}", lorem))
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.get(0).unwrap() * parts.get(1).unwrap())
        .reduce(|a, b| a + b)
        .unwrap();
    print!("The answer is: {}", gear_part_ratio_product_sum);

    Ok(())
}
fn check(entries:&Vec<char>, index:i64)->Option<usize>{
    let index = index as usize;
//    print!(" {} ", entries.get(index).unwrap_or(&'.'));
    if entries.get(index).unwrap_or(&'.') == &'*'{
        return Some(index)
    }
    None
}

fn check_neighbors(entries:&Vec<char>, cell:i64, line_length:i64)->Option<usize>{
        for index in vec![cell-1, cell-line_length, cell-line_length+1, cell-1, cell+line_length-1,
                          cell+line_length, cell+line_length+1, cell+1]{
            if !check(&entries, index).is_some(){
                continue;
            }
            return Some(index as usize);
        }
    None
}
