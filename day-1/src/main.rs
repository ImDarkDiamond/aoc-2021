use std::error::Error;

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let input = std::fs::read_to_string("../input/day-1.txt")?;

    // Part one!

    let lines: Vec<u64> = input
        .lines()
        .map(|string| string.parse::<u64>().expect("Failed to parse to number"))
        .collect();

    let mut increases = 0;

    for (index, number) in lines.iter().enumerate().skip(1) {
        let previous = lines.get(index - 1);

        if let Some(previous) = previous {
            if number > previous {
                increases += 1;
            }
        }
    }

    println!("There are {} increases in ocean depth", increases);

    // Part two!

    let mut larger_sums = 0;

    let mut last_measurement = 0;
    for (index, number) in lines.iter().enumerate() {
        let second = lines.get(index + 1);
        let third = lines.get(index + 2);

        if let Some(second) = second {
            if let Some(third) = third {
                let sum = number + second + third;

                if last_measurement != 0 && sum > last_measurement {
                    larger_sums += 1;
                }

                last_measurement = sum;
            }
        }
    }

    println!("There are {} larger sums", larger_sums);

    Ok(())
}
