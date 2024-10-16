use std::fs;

// 0: Red, 1: Green, 2: Blue
const CONFIGURATION: [u32; 3] = [12, 13, 14];

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lines: Vec<&str> = file.split("\n").map(|line| line.trim()).collect();

    let mut sum: u32 = 0;
    let mut power_sum: u32 = 0;

    for line in lines {
        let line: Vec<_> = line.split(": ").collect();

        let game_id: u32 = line[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

        let sets: Vec<_> = line[1].split("; ").collect();

        let mut possible = true;

        let mut max_red_revealed: u32 = 0;
        let mut max_green_revealed: u32 = 0;
        let mut max_blue_revealed: u32 = 0;

        for set in sets {
            let revealed: Vec<_> = set.split(", ").collect();

            let mut red_revealed: u32 = 0;
            let mut green_revealed: u32 = 0;
            let mut blue_revealed: u32 = 0;

            for reveal in revealed {
                let reveal: Vec<_> = reveal.split(" ").collect();

                let amount: u32 = reveal[0].parse().unwrap();

                match reveal[1] {
                    "red" => red_revealed += amount,
                    "green" => green_revealed += amount,
                    "blue" => blue_revealed += amount,
                    _ => {}
                }
            }

            max_red_revealed = max_red_revealed.max(red_revealed);
            max_green_revealed = max_green_revealed.max(green_revealed);
            max_blue_revealed = max_blue_revealed.max(blue_revealed);

            if red_revealed > CONFIGURATION[0] { possible = false; }
            if green_revealed > CONFIGURATION[1] { possible = false; }
            if blue_revealed > CONFIGURATION[2] { possible = false; }
        }

        power_sum += max_red_revealed * max_green_revealed * max_blue_revealed;

        if possible {
            println!("Game {} is possible", game_id);
            sum += game_id
        }
    }

    println!("Total sum of game ID's: {}", sum);
    println!("Total power sum: {}", power_sum);
}
