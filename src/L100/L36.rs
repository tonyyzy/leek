use crate::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut counter: u16;
        board.iter().all(|row| Self::is_valid(row.iter()))
            && (0..9).all(|col| Self::is_valid(board.iter().map(|row| &row[col])))
            && (0..3)
                .flat_map(|e| std::iter::repeat(e).zip(0..3))
                .all(|(r, c)| {
                    Self::is_valid(
                        (0..3)
                            .flat_map(|e| std::iter::repeat(e).zip(0..3))
                            .map(|(rr, cc)| &board[r * 3 + rr][c * 3 + cc]),
                    )
                })
    }

    fn is_valid<'a, T: Iterator<Item = &'a char>>(x: T) -> bool {
        let mut counter = 0;
        println!("Hello");
        for c in x {
            println!("{c}");
            match *c as u8 {
                n @ 49..=57 => {
                    let pos = n - 49;
                    if counter >> pos & 1 != 1 {
                        counter |= 1 << pos
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board))
    }

    #[test]
    fn t2() {
        let board: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false)
    }

    #[test]
    fn t3() {
        let board: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '.', '5', '.', '.', '1', '.'],
            vec!['.', '4', '.', '3', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '3', '.', '.', '1'],
            vec!['8', '.', '.', '.', '.', '.', '.', '2', '.'],
            vec!['.', '.', '2', '.', '7', '.', '.', '.', '.'],
            vec!['.', '1', '5', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '2', '.', '.', '.'],
            vec!['.', '2', '.', '9', '.', '.', '.', '.', '.'],
            vec!['.', '.', '4', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false)
    }
}
