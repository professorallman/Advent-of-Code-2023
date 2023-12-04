use std::fs::File;
use std::io::{BufReader, BufRead};
#[derive(Debug)]
struct Game{
    id:i32,
    rounds:Vec<Round>,
}
#[derive(Debug)]
struct Round{
    red:i32,
    green:i32,
    blue:i32,
}
enum Color{Red, Green, Blue}
impl Game{
    fn max(&self, color:Color)->i32{
        let mut max = 0;
        for round in self.rounds.iter(){
            match color{
                Color::Red => 
                    if round.red > max{
                        max = round.red;
                    },
                Color::Green =>
                    if round.green > max{
                        max = round.green;
                    },
                Color::Blue =>
                    if round.blue > max{
                        max = round.blue;
                    },
            }

        }
        max
    }
    fn max_red(&self)->i32{
        self.max(Color::Red)
    }
    fn max_green(&self)->i32{
        self.max(Color::Green)
    }
    fn max_blue(&self)->i32{
        self.max(Color::Blue)
    }
}
fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut games:Vec<Game> = Vec::new();
    for line in reader.lines(){
        let line = line?;
        let game_entry:Vec<&str> = line.split(':').collect();
        let game_id = game_entry[0].replace("Game ","").parse::<i32>().unwrap();
        let rounds:Vec<&str> = game_entry[1].split(';').collect();
        
        let mut game = Game{id:game_id, rounds:Vec::new()};
        for round in rounds{
            let cubes:Vec<Vec<&str>> = round.split(',')
                .map(| entry | entry.trim().split(' ').collect::<Vec<&str>>())
                .collect();
            let mut red_cube = 0;
            let mut green_cube = 0;
            let mut blue_cube = 0;
            for cube in cubes{
                let num = cube[0].parse::<i32>().unwrap_or(0);
                let color = cube[1];
                match color{
                    "red" => red_cube += num,
                    "green" => green_cube += num,
                    "blue" => blue_cube += num,
                    _ => (),
                }
                let round = Round{ red:red_cube, green:green_cube, blue:blue_cube};
                game.rounds.push(round);
            }

        }
        games.push(game);
        //println!("{:?}",rounds);
    }
    let game_id_total = games.iter().filter( |game| game.max_red() <= 12 &&
                                                 game.max_green() <= 13 &&
                                                 game.max_blue() <= 14)
                                .map( |game| game.id )
                                .reduce( |a, b| a + b);
                                
    println!("{:?}", game_id_total);

    let power_sum = games.iter().map( |game| game.max_red() * game.max_green() * game.max_blue())
            .reduce( |a, b| a + b);

    println!("Power sum: {:?}", power_sum);
    
    Ok(())
}
