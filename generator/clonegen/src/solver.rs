use crate::game::{Game, Direction};
use crate::bitboard::Bitboard;
use std::collections::{HashSet, VecDeque};

pub fn solve(mut game: Game) -> bool {
    let mut states = HashSet::new();
    let mut to_explore = VecDeque::new();
    states.insert((game.clones.clone(), game.boxes.clone()));
    to_explore.push_back((game.clones.clone(), game.boxes.clone()));

    while !to_explore.is_empty() || to_explore.len() > 10000 || states.len() > 10000 {
        let vertex = to_explore.pop_front();
        if let Some((clones, boxes)) = vertex {
            game.clones = clones;
            game.boxes = boxes;

            if game.is_solved() {
                return true;
            }

            for direction in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let mut new_game = game.clone();
                new_game.step(direction);

                if !states.contains(&(new_game.clones.clone(), new_game.boxes.clone())) {
                    states.insert((new_game.clones.clone(), new_game.boxes.clone()));
                    to_explore.push_back((new_game.clones.clone(), new_game.boxes.clone()))
                }
            }
        }

    }
    return false;
}
