const INPUT: &str = include_str!("../input/day06.txt");

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim_end()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect()
}


fn simulate_lanternfishs(numbers: &[usize], days: usize) -> usize {
    let mut age_groups = [0usize; 9];

    // first, we build add up all lanternfish of the same age and put them in a array, one entry
    // for each age.
    for age in numbers {
        age_groups[*age] += 1;
    }

    //print_age_group(&age_groups);

    // then, for each day we want to simulate
    for _ in 0..days {
        // we shift the array to the left (lanternfish in group 1 move to group 0),
        //  and simulate birth (next[8] = previous[0])
        //  and reset the birth countdown (next[6] = previous[7] + previous[0])
        age_groups = tick_age_group(age_groups);
        //print_age_group(&age_groups);
    }

    // then, just add up all currently existing lanternfish
    age_groups.iter().sum()
}

fn print_age_group(numbers: &[usize]) {
    println!(
        "[{:>6?},{:>6?},{:>6?},{:>6?},{:>6?},{:>6?},{:>6?},{:>6?},{:>6?}]",
        numbers[0],
        numbers[1],
        numbers[2],
        numbers[3],
        numbers[4],
        numbers[5],
        numbers[6],
        numbers[7],
        numbers[8]
    );
}

fn tick_age_group(yesterday: [usize; 9]) -> [usize; 9] {
    [
        yesterday[1],
        yesterday[2],
        yesterday[3],
        yesterday[4],
        yesterday[5],
        yesterday[6],
        yesterday[7] + yesterday[0],
        yesterday[8],
        yesterday[0],
    ]
}

fn main() {
    println!("part 1: {}", simulate_lanternfishs(&parse_input(INPUT), 80));
    println!("part 2: {}", simulate_lanternfishs(&parse_input(INPUT), 256));
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_day06_part1() {
        assert_eq!(350149, simulate_lanternfishs(&parse_input(INPUT), 80));
    }

    #[test]
    fn test_day06_part2() {
        assert_eq!(1590327954513, simulate_lanternfishs(&parse_input(INPUT), 256));
    }

    #[test]
    fn test_sample_part1() {
        let init = vec![3, 4, 3, 1, 2];
        let result = simulate_lanternfishs(&init, 80);
        assert_eq!(5934, result);
    }

    #[test]
    fn test_sample_part2() {
        let init = vec![3, 4, 3, 1, 2];
        let result = simulate_lanternfishs(&init, 256);
        assert_eq!(26984457539, result);
    }
}
