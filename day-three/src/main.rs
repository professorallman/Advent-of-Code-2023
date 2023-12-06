use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let schematics:Vec<Vec<char>> = reader.lines()
        .map( |line| line.unwrap().chars().collect::<Vec<_>>()).collect();
    
    for (i, row) in schematics.iter().enumerate(){
        for (j, char) in row.iter().enumerate(){
            if char.eq(&'.'){
                continue;
            }
            if !char.is_numeric() {
                continue;
            }
            if neighbor_checker(&schematics, i, j) {
                print!(" {} ", char);
            }
        }
    }
    Ok(())
}

fn neighbor_checker(grid:&[Vec<char>], i:usize,  j:usize)->bool{
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

fn check_cell(grid:&[Vec<char>], i:usize, j:usize)->bool{
    if let Some(row) = grid.get(i){
        if let Some(char) = row.get(j){
            if is_symbol(*char){
                return true;
            }
            
        }
    }
    false
}
fn is_symbol(char:char)->bool{
    !char.is_numeric() && !char.eq(&'.')
}
