use crate::bitboard::Bitboard;
use std::fmt;
use std::fmt::Formatter;

pub struct Game {
    rows: u8,
    columns: u8,
    pub clones: Bitboard,
    pub boxes: Bitboard,
    pub walls: Bitboard,
    pub win_position: Bitboard
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn to_tuple(&self) -> (i8, i8) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

impl Game {
    pub fn step(&mut self, direction: Direction) {
        let (dx, dy) = direction.to_tuple();

        let mut outer_iter: Box<dyn Iterator<Item = isize>> = if dx >= 0 {
            Box::new((0..(self.rows as isize)).rev())
        } else {
            Box::new(0..(self.rows as isize))
        };

        for row in outer_iter {
            let mut inner_iter: Box<dyn Iterator<Item = isize>> = if dy >= 0 {
                Box::new((0..self.columns as isize).rev())
            } else {
                Box::new(0..self.columns as isize)
            };
            for column in inner_iter {
                if 
                    (column - dx as isize) >= 0
                    && (column - dx as isize) < self.columns.into()
                    && (row - dy as isize) >= 0
                    && (row - dy as isize) < self.rows.into()
                {
                    if let Some(false) = self.walls.get(column as u8, row as u8) 
                        && let Some(true) = self.clones.get((column - dx as isize) as u8, (row - dy as isize) as u8) {
                            self.clones.set((column - dx as isize) as u8, (row - dy as isize) as u8, false);
                            self.clones.set(column as u8, row as u8, true);
                    }
                }
            }
        }
    }
    pub fn from_string(string: &str) -> Option<Self> {
        let mut lines = string.lines();

        let columns: u8;
        let rows: u8;

        let mut dimensions_str = lines.next()?.split_ascii_whitespace();

        columns = dimensions_str.next()?.parse::<u8>().ok()?;
        rows = dimensions_str.next()?.parse::<u8>().ok()?;
        if let Some(_) = dimensions_str.next() {
            return None;
        };

        let mut walls = Bitboard::new(columns, rows);
        let mut boxes = Bitboard::new(columns, rows);
        let mut clones = Bitboard::new(columns, rows);
        let mut win_position = Bitboard::new(columns, rows);
        for i in 0..rows {
            if let Some(line) = lines.next() {
                let mut chars = line.chars();
                for j in 0..columns {
                    match chars.next() {
                        Some(' ') => (),
                        Some('#') => {
                            walls.set(j, i, true);
                        },
                        Some('@') => {
                            clones.set(j, i, true);
                        },
                        Some('X') => {
                            boxes.set(j, i, true);
                        },
                        Some('.') => {
                            win_position.set(j, i, true);
                        },
                        _ => {
                            return None;
                        }
                    }
                }
                if let Some(_) = chars.next() {
                    return None;
                }
            } else {
                return None;
            }
        }
        if let Some(_) = lines.next() {
            return None;
        }
        Some(Self {
            rows,
            columns,
            walls,
            boxes,
            clones,
            win_position
        })
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut string = String::new();

        string.push_str(&self.columns.to_string());
        string.push(' ');
        string.push_str(&self.rows.to_string());
        string.push('\n');

        for i in 0..self.rows {
            for j in 0..self.columns {
                let to_push = if let Some(true) =  self.walls.get(j, i) {
                    '#'
                } else if let Some(true) = self.boxes.get(j, i) {
                    'X'
                } else if let Some(true) = self.clones.get(j, i){
                    '@'
                } else if let Some(true) = self.win_position.get(j, i) {
                    '.'
                } else {
                    ' '
                };
                string.push(to_push);
            }
            if i != self.rows - 1 {
                string.push('\n')
            }
        }
        write!(f, "{}", string)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::game::{Game, Direction};
    #[test]
    fn from_and_to() {
        let level_text = "5 5\n\
            ##.##\n\
            #   #\n\
            #@X@#\n\
            # X .\n\
            #####";
        println!("{:?}", level_text);
        let game = Game::from_string(level_text).unwrap();
        let new_string = format!("{:?}", game);

        assert_eq!(level_text, new_string);
    }

    #[test]
    fn borders_are_walls() {
        let level_text = "3 3\n   \n @ \n   ";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..3 {
            game.step(Direction::Right);
            println!("{:?}", game);
        }
        let level_after = "3 3\n   \n  @\n   ";
        assert_eq!(level_after, format!("{:?}", game));
    }

    #[test]
    fn walls_stop_clones() {
        let level_text = "3 3\n # \n#@#\n # ";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        game.step(Direction::Right);
        game.step(Direction::Left);
        game.step(Direction::Up);
        game.step(Direction::Down);
        assert_eq!(level_text, format!("{:?}", game));
    }

    fn clones_move_together() {
        let level_text = "3 3\n   \n @@\n   ";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        game.step(Direction::Right);
        game.step(Direction::Left);
        game.step(Direction::Up);
        let level_after = "3 3\n@@ \n   \n   ";
        assert_eq!(level_after, format!("{:?}", game));
    }
}
