use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::Add;
use std::usize;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut gear_parts:HashMap<(usize,usize), Vec<i64>> = HashMap::new();
    let entries:Vec<Vec<char>> = reader.lines()
        .map(|line| Vec::from_iter(line.unwrap().chars()))
        .collect();

    for (i,row) in entries.iter().enumerate(){
        let mut shift_amount:i64 = 0;
        for (j, entry) in row.iter().enumerate(){
            if shift_amount != 0{
                shift_amount = shift_amount - 1;
                continue;
            }
            if !entry.is_numeric(){
                continue;
            }
            let mut gear_indices:HashSet<(usize, usize)> = HashSet::new();
            if let Some(gear_index) = check_neighbors(&entries, i, j){
                gear_indices.insert(gear_index);
            }

            while j.add(shift_amount as usize) < row.len() && row.get(j.add(shift_amount as usize)).unwrap().is_numeric(){
                if let Some(gear_index) = check_neighbors(&entries, i, j.add(shift_amount as usize)){
                    gear_indices.insert(gear_index);
                }

                shift_amount = shift_amount + 1;
            }
            let parsed_num = row[j..(j.add(shift_amount as usize)) as usize].iter().collect::<String>().parse::<i64>().unwrap();

            for (gear_i, gear_j) in gear_indices.iter(){
                gear_parts.entry((*gear_i, *gear_j)).or_insert_with(Vec::new).push(parsed_num);
            }
        }
    }

    let gear_part_ratio_product_sum = 
    gear_parts
        .values()
        .into_iter()
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts.get(0).unwrap() * parts.get(1).unwrap())
        .reduce(|a, b| a + b)
        .unwrap();
    println!("The answer is: {}", gear_part_ratio_product_sum);

    Ok(())
}

fn check_neighbor(entries:&Vec<Vec<char>>, i:i64, j:i64)->Option<(usize,usize)>{
    if let Some(row) = entries.get(i as usize){
        if let Some(char) = row.get(j as usize){
            if char == &'*'{
                return Some((i as usize,j as usize));
            }
        }
    }
    None
}

fn check_neighbors(entries:&Vec<Vec<char>>, i:usize, j:usize) -> Option<(usize,usize)>{
    let i = i as i64;
    let j = j as i64;

    if let Some(upper_left) = check_neighbor(entries, i - 1, j - 1){
        return Some(upper_left);
    }
    if let Some(upper) = check_neighbor(entries, i-1, j){
        return Some(upper);
    }
    if let Some(upper_right) = check_neighbor(entries, i-1, j+1){
        return Some(upper_right);
    }
    if let Some(left) = check_neighbor(entries, i, j-1){
        return Some(left);
    }
    if let Some(right) = check_neighbor(entries, i, j+1){
        return Some(right);
    }
    if let Some(down_left) = check_neighbor(entries, i+1, j-1){
        return Some(down_left);
    }
    if let Some(down) = check_neighbor(entries,i+1, j){
        return Some(down);
    }
    if let Some(down_right) = check_neighbor(entries, i+1, j+1){
        return Some(down_right);
    }
    None
}
