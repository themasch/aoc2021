#![feature(portable_simd)]

use std::fmt::Debug;
use std::ops::Range;
use std::simd::Simd;

const LINE_WIDTH: usize = 12;
//TODO: would be nice if we could figure these out ourself without killing performance
const NUM_LINES: usize = 1000;
const INPUT: &str = include_str!("../input/day03.txt");

fn main() {
    let res = dbg!(add_lines_simd(INPUT, NUM_LINES));
    println!("part1: {}", res.gamma as usize * res.epsilon as usize);
    let step2 = dbg!(step_two(INPUT));
    println!("part2: {}", step2.oxygen as usize * step2.co2 as usize);
}

#[derive(Debug, Clone, Copy)]
enum FollowMode {
    // follow the most common bit
    Most,
    // follow the least common bit
    Least,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StatResult {
    One,
    Zero,
}

pub fn step_two(input: &str) -> StepTwoResult {
    let numbers = parse_and_sort(input);

    let oxygen = dbg!(search_value(&numbers, FollowMode::Most));
    let co2 = dbg!(search_value(&numbers, FollowMode::Least));

    StepTwoResult { oxygen, co2 }
}

fn search_value(nums: &[u16], mode: FollowMode) -> u16 {
    let mut range = 0..nums.len();
    for round in 0..12 {
        //dbg!(range.clone());
        let bitmask = 1 << (11 - round);
        //println!("bitmask: {:#016b}", bitmask);
        //println!("first:   {:#016b}", nums[range.start]);
        //println!("last:    {:#016b}", nums[range.end-1]);
        let most_common = most_common_digit(&nums[range.clone()], bitmask);
        let split_point = find_split_point(&nums[range.clone()], bitmask, most_common);
        match (most_common, &mode) {
            (StatResult::One, FollowMode::Most) => range.start += split_point,
            (StatResult::One, FollowMode::Least) => range.end = range.start + split_point,
            (StatResult::Zero, FollowMode::Most) => range.end = range.start + split_point,
            (StatResult::Zero, FollowMode::Least) => range.start += split_point,
        };

        if range.len() <= 1 {
            return nums[range.start];
        }
    }
    dbg!(&range);
    for (idx, num) in nums[range].iter().enumerate() {
        println!("{:#02} {:#016b}", idx, num);
    }
    panic!("nothing found!");
}

fn find_split_point(numbers: &[u16], bitmask: u16, most_common: StatResult) -> usize {
    // when most_common is One, we know the upper half of the slice ([mid..end]) starts with one
    // so we need to look for the first one in the lower half ([0..mid-1])
    // when most_common is zero, we know the lower half of the slice starts with zero, so
    // we need to look for the first zero in the upper half
    let looking_for_one = most_common == StatResult::One;
    let num_len = numbers.len();
    let halfway_point = num_len - (num_len / 2);
    let mut search_range = if most_common == StatResult::One {
        0..halfway_point
    } else {
        halfway_point..num_len
    };

    while search_range.len() > 1 {
        if numbers[search_range.start] & bitmask == numbers[search_range.end - 1] & bitmask {
            // the first and last entry have the same value in that bit, so the full range should
            // have that bit set
            let all_one = numbers[search_range.start] & bitmask == bitmask;
            return match (all_one, looking_for_one) {
                (true, true) => search_range.start,
                (false, true) => search_range.end,
                (true, false) => search_range.start,
                (false, false) => search_range.end,
            };
        }

        // find the middle in our range, and look if the bit is set there
        let center = center_of_range(&search_range);
        let bit_set_at_center = numbers[center] & bitmask == bitmask;
        match (looking_for_one, bit_set_at_center) {
            (true, true) => search_range.end = center,
            (true, false) => search_range.start = center,
            (false, true) => search_range.end = center,
            (false, false) => search_range.start = center,
        };
    }

    search_range.end
}

#[inline(always)]
fn center_of_range(range: &Range<usize>) -> usize {
    let range_len = range.end - range.start;
    if range_len & 0x01 == 0x01 {
        range.start + (range_len / 2)
    } else {
        range.start + range_len - (range_len / 2)
    }
}

fn most_common_digit(numbers: &[u16], bit_for_round: u16) -> StatResult {
    let center = center_of_range(&(0..numbers.len()));
    let num_at_center = numbers[center];

    if num_at_center & bit_for_round == bit_for_round {
        // 1 is most common (or equal)
        StatResult::One
    } else {
        // 0 is most common
        StatResult::Zero
    }
}

fn parse_and_sort(input: &str) -> Vec<u16> {
    let mut vec: Vec<_> = input.lines().map(parse_binary_to_int).collect();
    vec.sort_unstable();
    vec
}

fn parse_binary_to_int(num: &str) -> u16 {
    debug_assert!(num.len() == 12);
    let bytes = num.as_bytes();
    mask_to_int([
        bytes[0] == b'1',
        bytes[1] == b'1',
        bytes[2] == b'1',
        bytes[3] == b'1',
        bytes[4] == b'1',
        bytes[5] == b'1',
        bytes[6] == b'1',
        bytes[7] == b'1',
        bytes[8] == b'1',
        bytes[9] == b'1',
        bytes[10] == b'1',
        bytes[11] == b'1',
    ])
}

#[derive(Debug)]
pub struct StepTwoResult {
    oxygen: u16,
    co2: u16,
}

#[derive(Debug)]
pub struct StepOneResult {
    gamma: u16,
    epsilon: u16,
}

fn mask_to_int(mask: [bool; 12]) -> u16 {
    return if mask[0] { 0x0800 } else { 0 }
        + if mask[1] { 0x0400 } else { 0 }
        + if mask[2] { 0x0200 } else { 0 }
        + if mask[3] { 0x0100 } else { 0 }
        + if mask[4] { 0x0080 } else { 0 }
        + if mask[5] { 0x0040 } else { 0 }
        + if mask[6] { 0x0020 } else { 0 }
        + if mask[7] { 0x0010 } else { 0 }
        + if mask[8] { 0x0008 } else { 0 }
        + if mask[9] { 0x0004 } else { 0 }
        + if mask[10] { 0x0002 } else { 0 }
        + if mask[11] { 0x0001 } else { 0 };
}

pub fn add_lines_naive(input: &str, _line_count: usize) -> StepOneResult {
    let mut one_count: [u16; 12] = [0; 12];
    let mut zero_count: [u16; 12] = [0; 12];
    for line in input.lines() {
        debug_assert!(line.len() == 12);
        for (idx, chr) in line.chars().enumerate() {
            match chr {
                '0' => zero_count[idx] += 1,
                '1' => one_count[idx] += 1,
                _ => panic!("incorrect char: {}", chr),
            };
        }
    }

    let one_value = mask_to_int([
        one_count[0] > zero_count[0],
        one_count[1] > zero_count[1],
        one_count[2] > zero_count[2],
        one_count[3] > zero_count[3],
        one_count[4] > zero_count[4],
        one_count[5] > zero_count[5],
        one_count[6] > zero_count[6],
        one_count[7] > zero_count[7],
        one_count[8] > zero_count[8],
        one_count[9] > zero_count[9],
        one_count[10] > zero_count[10],
        one_count[11] > zero_count[11],
    ]);
    let zero_value = mask_to_int([
        one_count[0] <= zero_count[0],
        one_count[1] <= zero_count[1],
        one_count[2] <= zero_count[2],
        one_count[3] <= zero_count[3],
        one_count[4] <= zero_count[4],
        one_count[5] <= zero_count[5],
        one_count[6] <= zero_count[6],
        one_count[7] <= zero_count[7],
        one_count[8] <= zero_count[8],
        one_count[9] <= zero_count[9],
        one_count[10] <= zero_count[10],
        one_count[11] <= zero_count[11],
    ]);

    StepOneResult {
        gamma: one_value,
        epsilon: zero_value,
    }
}

pub fn add_lines_simd(input: &str, line_count: usize) -> StepOneResult {
    let mut accumulator = Simd::<u16, 16>::splat(0u16);
    // if the result is equal to '0' * line_count, that means every line contains a zero
    // for the ones to win we need > 50% of digits to be a 1, therefor the result needs
    // to be more than (line_count / 2) greater than '0' * line_count;
    let half_cutoff =
        Simd::<u16, 16>::splat('0' as u16 * line_count as u16 + line_count as u16 / 2);
    debug_assert!(line_count * (LINE_WIDTH + 1) <= input.len());
    let input_bytes = input.as_bytes();
    for rowidx in 0..line_count {
        let row_slice = &input_bytes[rowidx * (LINE_WIDTH + 1)..(rowidx + 1) * (LINE_WIDTH + 1)];
        let add = Simd::from_array([
            row_slice[0] as u16,
            row_slice[1] as u16,
            row_slice[2] as u16,
            row_slice[3] as u16,
            row_slice[4] as u16,
            row_slice[5] as u16,
            row_slice[6] as u16,
            row_slice[7] as u16,
            row_slice[8] as u16,
            row_slice[9] as u16,
            row_slice[10] as u16,
            row_slice[11] as u16,
            0,
            0,
            0,
            0,
        ]);

        accumulator += add;
    }

    let res = accumulator.lanes_gt(half_cutoff);
    let num_res = if res.test(0) { 0x0800 } else { 0 }
        + if res.test(1) { 0x0400 } else { 0 }
        + if res.test(2) { 0x0200 } else { 0 }
        + if res.test(3) { 0x0100 } else { 0 }
        + if res.test(4) { 0x0080 } else { 0 }
        + if res.test(5) { 0x0040 } else { 0 }
        + if res.test(6) { 0x0020 } else { 0 }
        + if res.test(7) { 0x0010 } else { 0 }
        + if res.test(8) { 0x0008 } else { 0 }
        + if res.test(9) { 0x0004 } else { 0 }
        + if res.test(10) { 0x0002 } else { 0 }
        + if res.test(11) { 0x0001 } else { 0 };

    StepOneResult {
        gamma: num_res,
        epsilon: (!num_res) & 0x0FFF,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day03_part1() {
        let res = add_lines_simd(INPUT, NUM_LINES);
        assert_ne!(3912944, res.gamma as usize * res.epsilon as usize);
    }

    #[test]
    fn test_day03_part2() {
        let res = step_two(INPUT);
        assert_ne!(4996233, res.oxygen as usize * res.co2 as usize);
    }

    #[test]
    fn test_add_simd() {
        let input = "101010101010\n010101010101\n101010101010\n";

        let exp_a = 0b101010101010;
        let exp_b = 0b010101010101;

        let res = add_lines_simd(input, 3);
        assert_eq!(exp_a, res.gamma);
        assert_eq!(exp_b, res.epsilon);
    }

    #[test]
    fn test_naive() {
        let input = "101010101010\n010101010101\n101010101010\n";

        let exp_a = 0b101010101010;
        let exp_b = 0b010101010101;

        let res = add_lines_naive(input, 3);
        assert_eq!(exp_a, res.gamma);
        assert_eq!(exp_b, res.epsilon);
    }

    #[test]
    fn test_step_two() {
        let input = "001000010000\n111101111000\n101101011000\n101111011100\n101011010100\n011110111100\n001110011100\n111001110000\n100001000000\n110011100100\n000100001000\n010100101000\n";

        let res = step_two(&input);

        let exp_o = 0b101111011100;
        let exp_c = 0b010100101000;
        assert_eq!(exp_o, res.oxygen);
        assert_eq!(exp_c, res.co2);
    }
}

/*
00 0b000100001000  0
01 0b001000010000  1
02 0b001110011100  2
03 0b010100101000  3
04 0b011110111100  4
05 0b100001000000 05
06 0b101011010100 16
07 0b101101011000 2
08 0b101111011100 3
09 0b110011100100 4
10 0b111001110000 5
11 0b111101111000 6
*/
