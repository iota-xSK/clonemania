mod bitboard;
mod game;
mod solver;
use crate::solver::solve;
use crate::game::Game;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("need exactly one argument. the path of the board file");
    }
    let contents = fs::read_to_string(args[1].clone())
        .expect("Should have been able to read the file");
    let game = Game::from_string(&contents).expect("couldn't parse game");

    if let Some(stats) = solve(game, 30) {
        println!("it is solvable. Stats: {:?}", stats);
    } else {
        println!("it is not solvable");
    }
}
