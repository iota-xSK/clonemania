use crate::game::{Game, Direction};
use crate::bitboard::Bitboard;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct SolutionStats {
    pub depth: u32,
    pub states: u32,
    pub solutions: u32,
    pub min_solution_length: u32
}

pub fn solve(mut game: Game, max_depth: u32) -> Option<SolutionStats> {
    let mut states = HashSet::new();
    let mut to_explore = VecDeque::new();
    states.insert((game.clones.clone(), game.boxes.clone()));
    to_explore.push_back((game.clones.clone(), game.boxes.clone(), 0));
    
    let mut stats = SolutionStats {
        depth: 0,
        states: 1,
        solutions: 0,
        min_solution_length: u32::MAX
    };

    'outer:
        while !to_explore.is_empty() || to_explore.len() > 10000 || states.len() > 10000 {
        let vertex = to_explore.pop_front();
        if let Some((clones, boxes, depth)) = vertex {
            game.clones = clones;
            game.boxes = boxes;

            stats.depth = depth;
            if depth >= max_depth {
                break 'outer;
            }

            if game.is_solved() {
                stats.solutions += 1;
                if depth < stats.min_solution_length {
                    stats.min_solution_length = depth
                }
            }

            for direction in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
                let mut new_game = game.clone();
                new_game.step(direction);

                if !states.contains(&(new_game.clones.clone(), new_game.boxes.clone())) {
                    states.insert((new_game.clones.clone(), new_game.boxes.clone()));
                    to_explore.push_back((new_game.clones.clone(), new_game.boxes.clone(), depth + 1))
                }
            }
        }
        stats.states = states.len() as u32;
    }
    if stats.solutions >= 0 {
        return Some(stats);
    }
    return None;
}
