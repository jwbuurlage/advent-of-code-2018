use std::io::{self, Read};

fn main() {
    let input = 165061;

    let mut board: Vec<usize> = vec![3,7];

    let mut first = 0;
    let mut second = 1;

    loop {
        let sum = board[first] + board[second];
        if sum > 9 {
            board.push(sum / 10);
        }
        board.push(sum % 10);


        first = (first + 1 + board[first]) % board.len();
        second = (second + 1 + board[second]) % board.len();

        if board.len() > input + 10 {
            let result = &board[input..(input + 10)];
            println!("{:?}", result);
            return;
        }
    }


}
