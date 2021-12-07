const INPUT: &str = include_str!("../input/day01.txt");

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(numbers: &[u32]) -> usize {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        //.fold(0, |acc, (a, b)| acc + (if b > a { 1 } else { 0 }));
        .filter(|(a, b)| b > a)
        .count()
}

fn part2(numbers: &[u32]) -> usize {
    // build sliding windows
    let aggre = numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .zip(numbers.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect::<Vec<u32>>();

    aggre
        .iter()
        .zip(aggre.iter().skip(1))
        //        .fold(0, |acc, (a, b)| acc + (if b > a { 1 } else { 0 }));
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    let numbers = parse_input(INPUT);
    let res = part1(&numbers);
    println!("result 1: {}", res);

    let res = part2(&numbers);
    println!("result 2: {}", res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day01_part01_complete() {
        let expected_result = 1553;
        let result = part1(&parse_input(INPUT));

        assert_eq!(expected_result, result);
    }
    #[test]
    fn test_day01_part02_complete() {
        let expected_result = 1597;
        let result = part2(&parse_input(INPUT));

        assert_eq!(expected_result, result);
    }
}
