use crate::bitboard::Bitboard;
use std::fmt;
use std::fmt::Formatter;

pub struct Game {
    rows: u8,
    columns: u8,
    pub clones: Bitboard,
    pub boxes: Bitboard,
    pub walls: Bitboard,
    pub targets: Bitboard
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

        let outer_iter: Box<dyn Iterator<Item = isize>> = if dx >= 0 {
            Box::new(0..(self.rows as isize))
        } else {
            Box::new((0..(self.rows as isize)).rev())
        };

        for row in outer_iter {
            let inner_iter: Box<dyn Iterator<Item = isize>> = if dy >= 0 {
                Box::new(0..self.columns as isize)
            } else {
                Box::new((0..self.columns as isize).rev())
            };
            for column in inner_iter {
                if 
                    (column - dx as isize) >= 0
                    && (column - dx as isize) < self.columns.into()
                    && (row - dy as isize) >= 0
                    && (row - dy as isize) < self.rows.into()
                {
                    // pull clone into this spot if it is empty and there is a clone to be pulled
                    if let Some(false) = self.walls.get(column as u8, row as u8) 
                        && let Some(false) = self.clones.get(column as u8, row as u8)
                        && let Some(false) = self.boxes.get(column as u8, row as u8)
                        && let Some(true) = self.clones.get((column - dx as isize) as u8, (row - dy as isize) as u8) {
                            self.clones.set((column - dx as isize) as u8, (row - dy as isize) as u8, false);
                            self.clones.set(column as u8, row as u8, true);
                    }

                    // pull box into this spot if it is empty and there is a clone to push it
                    if (column - dx as isize) >= 0
                        && (column - dx as isize) < self.columns.into()
                        && (row - dy as isize) >= 0
                        && (row - dy as isize) < self.rows.into()
                        && (column - 2*dx as isize) >= 0
                        && (column - 2*dx as isize) < self.columns.into()
                        && (row - 2*dy as isize) >= 0
                        && (row - 2*dy as isize) < self.rows.into()
                        {
                            if let Some(true) = self.boxes.get((column - dx as isize) as u8, (row - dy as isize) as u8)
                                && let Some(false) = self.walls.get(column as u8, row as u8)
                                && let Some(false) = self.boxes.get(column as u8, row as u8)
                                && let Some(true) = self.clones.get((column - 2*dx as isize) as u8, (row - 2*dy as isize) as u8) {
                                    self.boxes.set(column as u8, row as u8, true);
                                    self.boxes.set((column - dx as isize) as u8, (row - dy as isize) as u8, false);
                            }
                    }
                }
            }
        }
    }
    pub fn is_solved(&self) -> bool {
        let mut is_solved = true;
        for column in 0..self.columns {
            for row in 0..self.rows {
                if let Some(true) = self.targets.get(column, row)
                    && let Some(false) = self.boxes.get(column, row) {
                        is_solved = false;
                        return is_solved;
                }
            }
        }
        return is_solved;
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
        let mut targets = Bitboard::new(columns, rows);
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
                            targets.set(j, i, true);
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
            targets
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
                } else if let Some(true) = self.targets.get(j, i) {
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

    #[test]
    fn clones_move_together() {
        let level_text = "3 3\n   \n @@\n   ";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        game.step(Direction::Right);
        game.step(Direction::Left);
        game.step(Direction::Up);
        let level_after = "3 3\n@@ \n   \n   ";
        assert_eq!(level_after, format!("{:?}", game));
    }

    #[test]
    fn clones_push_boxes_right() {
        let level_text = "4 1\n@X  ";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..6 {
            game.step(Direction::Right);
        }
        let level_after = "4 1\n  @X";
        assert_eq!(level_after, format!("{:?}", game));
    }
    #[test]
    fn clones_push_boxes_left() {
        let level_text = "4 1\n X @";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..6 {
            game.step(Direction::Left);
        }
        let level_after = "4 1\nX@  ";
        assert_eq!(level_after, format!("{:?}", game));
    }

    #[test]
    fn clones_push_boxes_up() {
        let level_text = "1 4\n \nX\n \n@";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..6 {
            game.step(Direction::Up);
        }
        let level_after = "1 4\nX\n@\n \n ";
        assert_eq!(level_after, format!("{:?}", game));
    }

    #[test]
    fn clones_push_boxes_down() {
        let level_text = "1 4\n \n@\nX\n ";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..6 {
            game.step(Direction::Down);
        }
        let level_after = "1 4\n \n \n@\nX";
        assert_eq!(level_after, format!("{:?}", game));
    }

    #[test]
    fn clones_dont_push_multiple_boxes() {
        let level_text = "4 1\nXX @";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..6 {
            game.step(Direction::Left);
        }
        let level_after = "4 1\nXX@ ";
        assert_eq!(level_after, format!("{:?}", game));
    }

    #[test]
    fn clones_stop_each_other() {
        let level_text = "4 1\nXX@@";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..6 {
            game.step(Direction::Left);
        }
        let level_after = "4 1\nXX@@";
        assert_eq!(level_after, format!("{:?}", game));
    }
    #[test]
    fn walls_stop_boxes() {
        let level_text = "4 1\n#X@ ";
        let mut game = Game::from_string(level_text).expect("couldn't parse game");
        for _ in 0..6 {
            game.step(Direction::Left);
        }
        assert_eq!(level_text, format!("{:?}", game));
    }
}
