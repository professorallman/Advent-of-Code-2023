use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{cell::RefCell, rc::Rc};
use regex::Regex;
use itertools::Itertools;

struct Entry{
    id:usize,
    value:String,
    is_part:bool
}
impl Entry {
    pub fn set_is_part(&mut self, is_part:bool){
        self.is_part = is_part;
    }
}
fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let regex = Regex::new(r#"\W|\d*"#).unwrap();
    
    let schematics = reader
        .lines()
        .map( |line| regex.captures_iter(&line.unwrap()).map( |cap | String::from(cap.get(0).unwrap().as_str())).collect::<Vec<_>>())
        .map( |entries|{
            let mut row:Vec<Rc<RefCell<Entry>>> = Vec::new();
            entries.iter().enumerate().for_each( |(id,entry)|{
                let e = Rc::new(RefCell::new(Entry{id, value:entry.to_string(), is_part:false}));
                for _ in 0..entry.len(){
                    row.push(e.clone());
                }
            });
            row
        })
        .collect::<Vec<_>>();
    
    let data:Vec<Vec<String>> = schematics
        .iter()
        .map(|row| row.iter().map(|rc| rc.borrow().value.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    for (i, row) in data.iter().enumerate(){
        for (j,entry) in row.iter().enumerate(){
            if let Some(char) = entry.chars().next(){
                if char.eq(&'.'){
                 continue;
                }
                if !char.is_numeric() {
                 continue;
                }
            }
            if neighbor_checker(&data, i, j) {
                schematics.get(i).unwrap().get(j).unwrap().clone().borrow_mut().set_is_part(true);
            }
        }
    }

    let part_number_sum = schematics.iter()
        .map(|row| row.iter()
            .map(|rc| rc.borrow())
            .unique_by(|entry| entry.id)
            .filter(|entry| entry.is_part)
            .map(|entry|entry.value.parse::<i32>().unwrap())
            .reduce(|a,b| a + b)
            .unwrap()
            )
        .reduce(|a,b| a + b)
        .unwrap();

    println!("\n\nSum of all part numbers is: {}", part_number_sum);
    
    Ok(())
}

fn neighbor_checker(grid:&Vec<Vec<String>>, i:usize,  j:usize)->bool{
    if i >= grid.len() || j >= grid.len(){
        return false;
    }
    if let (Some(i_index), Some(j_index)) = (i.checked_sub(1), j.checked_sub(1)){
        if check_cell(grid, i_index, j_index){
            return true;
        }
    }
    if let (Some(i_index), Some(j_index)) = (i.checked_sub(1), Some(j)){
        if check_cell(grid, i_index, j_index){
            return true;
        }
    }

    if let (Some(i_index), Some(j_index)) = (i.checked_sub(1), j.checked_add(1)){
        if check_cell(grid, i_index, j_index){
            return true;
        }
    }


    if let (Some(i_index), Some(j_index)) = (Some(i), j.checked_sub(1)){
        if check_cell(grid, i_index, j_index){
            return true;
        }
    }

    if let (Some(i_index), Some(j_index)) = (Some(i), j.checked_add(1)){
        if check_cell(grid, i_index, j_index){
            return true;
        }
    }

    if let (Some(i_index), Some(j_index)) = (i.checked_add(1), j.checked_sub(1)){
        if check_cell(grid, i_index, j_index){
            return true;
        }
    }

    if let (Some(i_index), Some(j_index)) = (i.checked_add(1), Some(j)){
        if check_cell(grid, i_index, j_index){
            return true;
        }
    }

    if let (Some(i_index), Some(j_index)) = (i.checked_add(1), j.checked_add(1)){
        if check_cell(grid, i_index, j_index){
         return true;
        }
    }

    false
}

fn check_cell(grid:&Vec<Vec<String>>, i:usize, j:usize)->bool{
    if let Some(row) = grid.get(i){
        if let Some(entry) = row.get(j){
            if let Some(char) = entry.chars().next(){
                if is_symbol(char){
                    return true;
                }
            }
        }
    }
    false
}
fn is_symbol(char:char)->bool{
    !char.is_numeric() && !char.eq(&'.')
}
