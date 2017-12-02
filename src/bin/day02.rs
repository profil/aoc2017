const DATA: &str = include_str!("../../data/day02.txt");

fn main() {
    let data: Vec<Vec<u32>> = DATA.lines()
        .map(|row| {
            row.split_whitespace().map(|c| c.parse().unwrap()).collect()
        })
        .collect();

    println!(
        "Part one: {}\nPart two: {}",
        part_one(&data),
        part_two(&data)
    );
}

fn part_one(data: &Vec<Vec<u32>>) -> u32 {
    data.iter()
        .map(|row| {
            let max = row.iter().max().unwrap();
            let min = row.iter().min().unwrap();
            max - min
        })
        .sum()
}

fn part_two(data: &Vec<Vec<u32>>) -> u32 {
    data.iter()
        .map(|row| {
            for a in row {
                for b in row {
                    if a != b && a % b == 0 {
                        return a / b;
                    }
                }
            }
            0
        })
        .sum()
}
