use std::collections::HashSet;

#[derive(Debug)]
struct Board {
    numbers: [[i32; BOARD_SIZE]; BOARD_SIZE],
    bingo: bool,
}

const BOARD_SIZE: usize = 5;

impl Board {
    pub fn parse(rows: &[String]) -> Board {
        assert!(rows.len() == BOARD_SIZE);

        let mut numbers = [[0; BOARD_SIZE]; BOARD_SIZE];
        let mut row_index = 0;
        for row in rows {
            let nums: Vec<i32> = row
                .split(' ')
                .filter_map(|n| {
                    if n == "" {
                        None
                    } else {
                        Some(n.parse::<i32>().unwrap())
                    }
                })
                .collect();

            assert!(nums.len() == BOARD_SIZE);

            for i in 0..BOARD_SIZE {
                numbers[row_index][i] = nums[i];
            }

            row_index += 1;
        }

        Board {
            numbers,
            bingo: false,
        }
    }

    fn rows(&self) -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
        self.numbers.clone()
    }

    fn cols(&self) -> [[i32; BOARD_SIZE]; BOARD_SIZE] {
        let mut cols = [[0; BOARD_SIZE]; BOARD_SIZE];
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                cols[c][r] = self.numbers[r][c];
            }
        }
        cols
    }

    /// Get flat list of all numbers in board
    fn flat(&self) -> HashSet<i32> {
        self.numbers.iter().cloned().flatten().collect()
    }

    /// Calculate score
    pub fn score(&self, draws: &Vec<i32>) -> i32 {
        let draw_set: HashSet<i32> = draws.iter().cloned().collect();
        self.flat().difference(&draw_set).sum()
    }

    fn is_bingo_inner(row_or_col: [i32; 5], draws: Vec<i32>) -> bool {
        let draw_set: HashSet<i32> = draws.into_iter().collect();
        let row_or_col_set: HashSet<i32> = row_or_col.into_iter().collect();
        // Are all numbers in the row or column drawn?
        row_or_col_set.difference(&draw_set).count() == 0
    }

    pub fn is_bingo(&self, draws: &Vec<i32>) -> bool {
        self.rows()
            .iter()
            .any(|&row| Board::is_bingo_inner(row, draws.iter().cloned().collect()))
            || self
                .cols()
                .iter()
                .any(|&col| Board::is_bingo_inner(col, draws.iter().cloned().collect()))
    }
}

pub fn go4a(lines: Vec<String>) {
    let draws: Vec<i32> = lines[0]
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    for board in lines.split(|l| l == "").skip(1) {
        boards.push(Board::parse(board));
    }

    for draw in 0..draws.len() {
        let drawn_numbers: Vec<i32> = draws.iter().take(draw).cloned().collect();
        let mut score = 0;

        for board in &mut boards {
            if !board.bingo && board.is_bingo(&drawn_numbers) {
                board.bingo = true;
                score = board.score(&drawn_numbers);
                println!("Board wins with score {}", score);
                println!("Last called number {}", drawn_numbers.last().unwrap());
                // break;
            }
        }

        if score > 0 {
            // break;
        }
    }
}
