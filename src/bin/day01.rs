const DATA: &str = include_str!("../../data/day01.txt");

fn main() {
    let data: Vec<u32> = DATA.chars().filter_map(|c| c.to_digit(10)).collect();
    println!(
        "Part one: {}\nPart two: {}",
        captcha(&data, 1),
        captcha(&data, data.len() / 2)
    );
}

fn captcha(numbers: &Vec<u32>, skip: usize) -> u32 {
    numbers
        .iter()
        .zip(numbers.iter().cycle().skip(skip))
        .filter_map(|(x, y)| if x == y { Some(x) } else { None })
        .sum()
}
