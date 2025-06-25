use std::env;

use rustdoku::Board;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <board_string>", args[0]);
        std::process::exit(1);
    }

    let board_string = &args[1];

    let mut board = Board::new();
    match board.set(board_string) {
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
        _ => {}
    }

    board.print();
    board.solve();
    board.print();
}
