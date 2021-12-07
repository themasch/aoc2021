#[derive(Debug, Copy, Clone)]
enum Direction {
    Forward,
    Up,
    Down,
}

const INPUT: &str = include_str!("../input/day02.txt");

fn parse_input(input: &str) -> Vec<(Direction, u8)> {
    input
        .lines()
        .filter(|line| line.contains(' '))
        .map(parse_line2)
        .collect::<Vec<_>>()
}

fn main() {
    let sum = get_part1(parse_input(INPUT).iter());
    println!("Part 1: {} * {} = {}", sum.0, sum.1, sum.0 * sum.1);

    let sum2 = get_part2(parse_input(INPUT).iter());
    println!("Part 1: {} * {} = {}", sum2.0, sum2.1, sum2.0 * sum2.1);
}

#[allow(dead_code)]
fn parse_line(line: &str) -> (Direction, u8) {
    let split: Vec<&str> = line.split_whitespace().take(2).collect();
    let dir = match split[0] {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!("unsupported direction {}", split[0]),
    };

    let count: u8 = split[1].parse().unwrap();

    (dir, count)
}

fn parse_line2(line: &str) -> (Direction, u8) {
    let mut chars = line.chars();
    let dir_char = chars.next().unwrap();
    let count_char = chars.last().unwrap();
    let dir = match dir_char {
        'f' => Direction::Forward,
        'u' => Direction::Up,
        'd' => Direction::Down,
        _ => panic!("unsupported direction {}", line),
    };

    let count = count_char.to_digit(10).unwrap();
    debug_assert!(count < 10);

    (dir, count as u8)
}

fn get_part1<'a>(inp: impl Iterator<Item = &'a (Direction, u8)>) -> (usize, usize) {
    inp.fold((0, 0), |(ah, av), &(dir, count)| match dir {
        Direction::Forward => (ah + count as usize, av),
        Direction::Up => (ah, av - count as usize),
        Direction::Down => (ah, av + count as usize),
    })
}

fn get_part2<'a>(inp: impl Iterator<Item = &'a (Direction, u8)>) -> (isize, isize, isize) {
    inp.fold((0, 0, 0), |(ah, av, aim), &(dir, count)| match dir {
        Direction::Forward => (ah + count as isize, av as isize + aim * count as isize, aim),
        Direction::Up => (ah, av, aim - count as isize),
        Direction::Down => (ah, av, aim + count as isize),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day02_part1() {
        let p1_res = get_part1(parse_input(INPUT).iter());
        assert_eq!(2073315, p1_res.0 * p1_res.1);
    }

    #[test]
    fn test_day02_part2() {
        let p2_res = get_part2(parse_input(INPUT).iter());
        assert_eq!(1840311528, p2_res.0 * p2_res.1);
    }

    #[test]
    fn test_part2() {
        use Direction::*;
        let input = vec![
            (Forward, 5),
            (Down, 5),
            (Forward, 8),
            (Up, 3),
            (Down, 8),
            (Forward, 2),
        ];

        let res = get_part2(input.iter());

        assert_eq!((15, 60, 10), res);
    }
}
