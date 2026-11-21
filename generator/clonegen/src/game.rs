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
    use crate::game::Game;
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
}
