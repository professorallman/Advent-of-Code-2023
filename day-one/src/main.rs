use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total:i64 = 0;
    for line in reader.lines(){
        let line = line?;
        let first_char = first_numeric_char(line.clone(), false);
        let last_char = last_numeric_char(line.clone(), false);
        
        let num = String::from_iter([first_char, last_char]).parse::<i64>().unwrap_or(0);
        total += num;    

    }
    print!("Answer: {} \n\n\n\n", total);


        let file = File::open("input.txt")?;
        let reader = BufReader::new(file);
        total = 0;
        for line in reader.lines(){
            let line = line?;
            let first_char = first_numeric_char(line.clone(), true);
            let last_char = last_numeric_char(line.clone(), true);
        
            let num = String::from_iter([first_char, last_char]).parse::<i64>().unwrap_or(0);
            total += num;
        }

    print!("Answer 2: {} ", total);

    Ok(())
}
enum NumberWords{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl std::fmt::Display for NumberWords{
    fn fmt(& self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self{
            NumberWords::One => write!(f, "One"),
            NumberWords::Two => write!(f, "Two"),
            NumberWords::Three => write!(f, "Three"),
            NumberWords::Four=> write!(f, "Four"),
            NumberWords::Five => write!(f, "Five"),
            NumberWords::Six => write!(f, "Six"),
            NumberWords::Seven => write!(f, "Seven"),
            NumberWords::Eight => write!(f, "Eight"),
            NumberWords::Nine => write!(f, "Nine"),
        }
    }
}

impl FromStr for NumberWords{
    type Err = ();
    fn from_str(input: &str) ->Result<NumberWords, Self::Err>{
        match input {
            "one" => Ok(NumberWords::One),
            "eno" => Ok(NumberWords::One),
            "two" => Ok(NumberWords::Two),
            "owt" => Ok(NumberWords::Two),
            "three" => Ok(NumberWords::Three),
            "eerth" => Ok(NumberWords::Three),
            "four" => Ok(NumberWords::Four),
            "ruof" => Ok(NumberWords::Four),
            "five" => Ok(NumberWords::Five),
            "evif" => Ok(NumberWords::Five),
            "six" => Ok(NumberWords::Six),
            "xis" => Ok(NumberWords::Six),
            "seven" => Ok(NumberWords::Seven),
            "neves" => Ok(NumberWords::Seven),
            "eight" => Ok(NumberWords::Eight),
            "thgie" => Ok(NumberWords::Eight),
            "nine" => Ok(NumberWords::Nine),
            "enin" => Ok(NumberWords::Nine),
            _ => Err(())
        }
    }
}
fn first_numeric_char(line:String, include_number_words:bool) -> char{
    let chars = line.chars();
    for (i, char) in chars.enumerate(){
        if char.is_numeric(){
            return char;
        }
        if !include_number_words || i + 3 >= line.len(){
            continue;
        }
        if let Ok(number_word) = NumberWords::from_str(&line[i..i+3]){
            match number_word{
                NumberWords::One => return '1',
                NumberWords::Two => return '2',
                NumberWords::Six => return '6',
                _ => (),
            }

        }

        if i + 4 >= line.len(){
            continue;
        }

        if let Ok(number_word) = NumberWords::from_str(&line[i..i+4]){
            match number_word{
                NumberWords::Four => return '4',
                NumberWords::Five => return '5',
                NumberWords::Nine => return '9',
                _ => (),
            }
        }

        if i + 5 >= line.len(){
            continue;
        }
        if let Ok(number_word) = NumberWords::from_str(&line[i..i+5]){
            match number_word{
                NumberWords::Three => return '3',
                NumberWords::Seven => return '7',
                NumberWords::Eight => return '8',
                _ => (),
            }
        }

    }
    '0'
}
fn last_numeric_char(line:String, include_other_words:bool)->char{
    let chars = line.chars().rev();
    for (i, char) in chars.enumerate(){
        if char.is_numeric(){
            return char;
        }

        if !include_other_words || i + 3 >= line.len(){
            continue;
        }
        let j = line.len() - i;

                if let Ok(number_word) = NumberWords::from_str(&line[j-3..j]){
            match number_word{
                NumberWords::One => return '1',
                NumberWords::Two => return '2',
                NumberWords::Six => return '6',
                _ => (),
            }

        }

        if i + 4 >= line.len(){
            continue;
        }

        if let Ok(number_word) = NumberWords::from_str(&line[j-4..j]){
            match number_word{
                NumberWords::Four => return '4',
                NumberWords::Five => return '5',
                NumberWords::Nine => return '9',
                _ => (),
            }
        }

        if i + 5 >= line.len(){
            continue;
        }
        if let Ok(number_word) = NumberWords::from_str(&line[j-5..j]){
            match number_word{
                NumberWords::Three => return '3',
                NumberWords::Seven => return '7',
                NumberWords::Eight => return '8',
                _ => (),
            }
        }
    }

    '0'
}
