#[derive(Eq, Hash, PartialEq, Clone)]
pub struct Bitboard {
    columns: u8,
    rows: u8,
    board: Vec<u8>
}

impl Bitboard {
    pub fn new(columns: u8, rows: u8) -> Bitboard {
        let bytesnum = (columns as usize) * (rows as usize) / 8 + 1;
        let mut board = Vec::with_capacity(bytesnum);

        for _ in 0..(bytesnum) {
            board.push(0)
        }

        return Bitboard {
            board,
            columns,
            rows,
        }
    }

    pub fn set(&mut self, column: u8, row: u8, value: bool) {
        if column >= self.columns || row >= self.rows {
            return;
        }

        let bit_idx = (row as usize) * (self.columns as usize) + (column as usize); 
        let byte_idx = bit_idx / 8;
        let bit_in_byte_idx = bit_idx % 8;

        let mut byte = self.board[byte_idx];
        if value {
            byte = byte | (1 << bit_in_byte_idx);
        } else {
            byte = byte & (0xff ^ (1<<bit_in_byte_idx));
        }
        self.board[byte_idx] = byte;
    }

    pub fn get(&self, column: u8, row: u8) -> Option<bool> {
        if column >= self.columns || row >= self.rows {
            return None;
        }

        let bit_idx = (row as usize) * (self.columns as usize) + (column as usize); 
        let byte_idx = bit_idx / 8;
        let bit_in_byte_idx = bit_idx % 8;

        let byte = self.board[byte_idx];
        return Some(byte & (1 << bit_in_byte_idx) != 0);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get() {
        for rows in 1..=20 {
            for columns in 1..=20 {
                println!("rows: {:?}, columns: {:?}", rows, columns);
                let mut board = Bitboard::new(columns, rows);
                for i in 0..columns {
                    for j in 0..rows {
                        board.set(i, j, (i+j)%2 == 0)
                    }
                }

                for i in 0..columns {
                    for j in 0..rows {
                        let value = board.get(i, j);

                        assert!(value.is_some());
                        assert_eq!(value.unwrap(), (i+j)%2 == 0);
                    }
                }
            }
        }
    }
}
