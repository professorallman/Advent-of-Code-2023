
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let winning_number_totals = reader
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let parts = line
                .split("|")
                .collect::<Vec<_>>().clone();
            let round_parts = parts[0].split(":").collect::<Vec<_>>();

            let round = round_parts[0];
            let winning_numbers = round_parts[1]
                .trim()
                .split(" ")
                .filter(|line| line.parse::<i64>().is_ok())
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let your_numbers = parts[1]
                .trim()
                .split(" ")
                .filter(|line| line.parse::<i64>().is_ok())
                .map(|part| part.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            println!("{},{:?},{:?}", round, winning_numbers, your_numbers);

            (round.to_owned(), winning_numbers.to_owned(), your_numbers.to_owned())
        })
        .map(|(_,winning_numbers, your_numbers)|{
            let mut points = 0;
            your_numbers.iter()
                .filter(|number| winning_numbers.contains(number))
                .for_each(|_|{ if points == 0 {
                    points = 1;
                }else{
                    points = points * 2
                }
                });

            points
        })
        .reduce(|a, b| a + b).unwrap();
    println!("The answer in: {:?}", winning_number_totals);
    Ok(())

//    for line in reader.lines(){

    //}
}
