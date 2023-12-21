
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::Add;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let data = reader
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

            (round.to_owned(), winning_numbers.to_owned(), your_numbers.to_owned())
        });
        let mut games:Vec<Game> = Vec::new();
        let winning_number_totals = data.map(|(_,winning_numbers, your_numbers)|{
            let mut points = 0;
            let mut number_of_dubs = 0;

            your_numbers.iter()
                .filter(|number| winning_numbers.contains(number))
                .inspect(|_| number_of_dubs = number_of_dubs + 1)
                .for_each(|_|{ if points == 0 {
                    points = 1;
                }else{
                    points = points * 2
                }
                });

            games.push(Game { instances: 0, number_of_dubs });

            points
        })
        .reduce(|a, b| a + b).unwrap();
    println!("The answer in: {:?}", winning_number_totals);
    for  i in 0..games.len(){
        play_game(&mut games, i);
    }
    println!("The games:\n{:?}", games);
    Ok(())
}
#[derive(Debug)]
struct Game{
    instances:i32,
    number_of_dubs:i32
}
fn play_game(games:&mut Vec<Game>, i:usize){
    let game = games.get_mut(i).unwrap();
    game.instances = game.instances + 1;
    let number_of_dubs:usize = game.number_of_dubs.try_into().expect("should work");
    println!("Playing game {}, num dubs:{}", i, number_of_dubs);
    for j in i.add(1)..number_of_dubs.add(1){
        play_game(games, j);
    }
}
