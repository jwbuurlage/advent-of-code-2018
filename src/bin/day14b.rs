use std::io::{self, Read};

fn main() {
    let mut board: Vec<usize> = vec![3,7];

    let mut first = 0;
    let mut second = 1;

    let input = [1,6,5,0,6,1];

    loop {
        let sum = board[first] + board[second];
        if sum > 9 {
            board.push(sum / 10);
        }

        if board.len() >= input.len() {
            if board[(board.len()-input.len())..board.len()] == input {
                println!("{}", board.len() - input.len());
                return;
            }
        }

        board.push(sum % 10);


        first = (first + 1 + board[first]) % board.len();
        second = (second + 1 + board[second]) % board.len();

        if board.len() >= input.len() {
            if board[(board.len()-input.len())..board.len()] == input {
                println!("{}", board.len() - input.len());
                return;
            }
        }
    }


}
