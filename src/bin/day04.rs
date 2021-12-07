const INPUT: &str = include_str!("../input/day04.txt");

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<u8>>, // [row][col] would be nice maybe? [[u8; 5]; 5],
    marked: Vec<Vec<bool>>,
}

impl Board {
    fn is_done(&self) -> bool {
        // any row done?
        if (0..5).any(|row| self.is_row_complete(row)) {
            return true;
        }

        (0..5).any(|col| self.is_col_complete(col))
    }

    fn is_row_complete(&self, row_index: usize) -> bool {
        self.marked[row_index].iter().all(|&v| v)
    }

    fn is_col_complete(&self, col_idx: usize) -> bool {
        for row in self.marked.iter() {
            if !row[col_idx] {
                return false;
            }
        }

        true
    }

    fn mark_number(&mut self, number: u8) {
        for (row_idx, row) in self.numbers.iter().enumerate() {
            if row.contains(&number) {
                for (col_idx, &num) in row.iter().enumerate() {
                    if num == number {
                        self.marked[row_idx][col_idx] = true;
                        //TODO: can a number appear more than once? can we exit here?
                    }
                }
            }
        }
    }

    fn get_unmarked(&self) -> Vec<u8> {
        self.marked
            .iter()
            .enumerate()
            .flat_map(|(row_index, marked_row)| {
                marked_row
                    .iter()
                    .enumerate()
                    .filter(|(_, &marked)| !marked)
                    .map(move |(col_index, _)| self.numbers[row_index][col_index])
            })
            .collect()
    }
}

type NumbersDrawn = Vec<u8>;

#[derive(Debug)]
struct Input {
    numbers_drawn: NumbersDrawn,
    boards: Vec<Board>,
}

fn parse_input(input: &str) -> Input {
    let end_of_draw_line = input.find('\n').unwrap();
    let numbers_drawn = dbg!(parse_draw_list(&input[0..end_of_draw_line]));

    let remaining = &input[end_of_draw_line + 1..];
    let boards = parse_boards(remaining.lines().collect::<Vec<_>>().chunks(6));

    Input {
        numbers_drawn,
        boards,
    }
}
use std::slice::Chunks;
fn parse_boards(board_chunks: Chunks<&str>) -> Vec<Board> {
    board_chunks
        .filter(|board| board.len() == 6)
        .map(|board| {
            let num = board
                .iter()
                .skip(1)
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<_>>();

            Board {
                numbers: num,
                marked: vec![vec![false; 5]; 5],
            }
        })
        .collect()
}

fn parse_draw_list(input: &str) -> NumbersDrawn {
    input
        .split(',')
        .map(|num| num.trim().parse().unwrap())
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let Input {
        numbers_drawn,
        mut boards,
    } = parse_input(input);

    for num in numbers_drawn.iter() {
        for board in boards.iter_mut() {
            board.mark_number(*num);
            if board.is_done() {
                let unmarked_sum: usize =
                    board.get_unmarked().iter().map(|&num| num as usize).sum();
                let winning_number = *num as usize;

                return unmarked_sum * winning_number;
            }
        }
    }

    panic!("no one won!");
}

fn solve_part2(input: &str) -> usize {
    let Input {
        numbers_drawn,
        mut boards,
    } = parse_input(input);

    dbg!(boards.len());
    dbg!(numbers_drawn.len());

    let all_results: Vec<(usize, usize, usize)> = numbers_drawn
        .iter()
        .flat_map(|input_num| {
            let mut winners = vec![];

            for (bidx, board) in boards.iter_mut().enumerate() {
                if board.is_done() {
                    continue;
                }

                board.mark_number(*input_num);
                if board.is_done() {
                    let unmarked_sum: usize =
                        board.get_unmarked().iter().map(|&num| num as usize).sum();
                    let winning_number = *input_num as usize;
                    winners.push((unmarked_sum, winning_number, bidx));
                }
            }
            winners
        })
        .collect();

    dbg!(all_results.len());
    //dbg!(all_results.clone());

    all_results.last().map(|(x, y, _)| x * y).unwrap()
}

fn main() {
    println!("part1: {}", solve_part1(INPUT));
    println!("part2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day04_part1() {
        assert_eq!(67716, solve_part1(INPUT));
    }

    #[test]
    fn test_day04_part2() {
        assert_eq!(1830, solve_part2(INPUT));
    }

    #[test]
    fn test_board_is_not_done_via_row() {
        let b = Board {
            numbers: vec![],
            marked: vec![
                vec![false; 5],
                vec![false; 5],
                vec![true, true, true, true, false],
                vec![false; 5],
                vec![false; 5],
                vec![false; 5],
            ],
        };

        assert!(!b.is_done());
    }

    #[test]
    fn test_board_is_done_via_row() {
        let b = Board {
            numbers: vec![],
            marked: vec![
                vec![false; 5],
                vec![false; 5],
                vec![true; 5],
                vec![false; 5],
                vec![false; 5],
                vec![false; 5],
            ],
        };

        assert!(b.is_done());
        assert!(b.is_row_complete(2));
    }

    #[test]
    fn test_board_is_not_done_via_col() {
        let b = Board {
            numbers: vec![],
            marked: vec![
                vec![true, true, false, true, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
            ],
        };

        assert!(!b.is_done());
    }

    #[test]
    fn test_board_is_done_via_col() {
        let b = Board {
            numbers: vec![],
            marked: vec![
                vec![true, true, true, true, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
            ],
        };

        assert!(b.is_done());
        assert!(b.is_col_complete(2));
    }

    #[test]
    fn test_board_get_unmarked() {
        let b = Board {
            numbers: vec![
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 10],
                vec![11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20],
                vec![21, 22, 23, 24, 25],
            ],
            marked: vec![
                vec![true, true, true, true, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
                vec![false, false, true, false, false],
            ],
        };

        assert_eq!(
            vec![5, 6, 7, 9, 10, 11, 12, 14, 15, 16, 17, 19, 20, 21, 22, 24, 25],
            b.get_unmarked()
        );
    }
}
