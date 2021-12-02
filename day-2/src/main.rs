use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let input = std::fs::read_to_string("../input/day-2.txt")?;

    // Part one!

    let lines = input.lines();

    let mut depth = 0;
    let mut horizontal = 0;

    for line in lines.clone() {
        let input = line.split(" ").collect::<Vec<&str>>();
        let action = input.first().unwrap();
        let num = input.get(1).unwrap().parse::<u64>()?;

        match *action {
            "forward" => {
                horizontal += num;
            }
            "down" => {
                depth += num;
            }
            "up" => {
                depth -= num;
            }
            _ => panic!("Unknown movement: {}", action),
        }
    }

    println!("Part one: {}", depth * horizontal);

    // Part two

    depth = 0;
    horizontal = 0;
    let mut aim = 0;

    for line in lines {
        let input = line.split(" ").collect::<Vec<&str>>();
        let action = input.first().unwrap();
        let num = input.get(1).unwrap().parse::<u64>()?;

        match *action {
            "forward" => {
                horizontal += num;
                depth += aim * num;
            }
            "down" => {
                aim += num;
            }
            "up" => {
                aim -= num;
            }
            _ => panic!("Unknown movement: {}", action),
        }
    }

    println!("Part two: {}", depth * horizontal);

    
    Ok(())
}
