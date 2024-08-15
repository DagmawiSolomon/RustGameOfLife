#[derive(Debug)]
pub enum Cell {
    ALIVE,
    DEAD,
}

pub struct Universe {
    width: u32,
    height: u32,
    cell: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.height + x) as usize
    }
    pub fn new(width: u32, height: u32) -> Universe {
        let mut cells: Vec<Cell> = Vec::new();
        for i in 0..(width * height) {
            cells.push(Cell::DEAD);
            if i % 31 == 0 {
                cells.push(Cell::ALIVE);
            }
        }
        Universe {
            width: width,
            height: height,
            cell: cells,
        }
    }

    fn get_left(&self, index: u32) -> Option<u32> {
        if index > 0 && index % self.width > 0 {
            Some(index - 1)
        } else {
            None
        }
    }

    fn get_right(&self, index: u32) -> Option<u32> {
        if index < self.width * self.height && index % self.width < self.width - 1 {
            Some(index + 1)
        } else {
            None
        }
    }

    fn get_top(&self, index: u32) -> Option<u32> {
        if index > 0 && index as i32 - self.width as i32 >= 0 {
            Some(index - self.width)
        } else {
            None
        }
    }

    fn get_bottom(&self, index: u32) -> Option<u32> {
        if index < self.width * self.height && index + self.width < self.height * self.width {
            Some(index + self.width)
        } else {
            None
        }
    }

    fn check_neighbours(&self, index: u32) -> u32 {
        fn is_alive(cell: Option<u32>, cells: &[Cell]) -> u32 {
            match cell {
                Some(i) => match cells[i as usize] {
                    Cell::ALIVE => 1,
                    _ => 0,
                },
                None => 0,
            }
        }

        let mut alive_count = 0;

        let directions = [
            self.get_left(index),
            self.get_right(index),
            self.get_top(index),
            self.get_bottom(index),
            self.get_top(index)
                .and_then(|top_index| self.get_left(top_index)),
            self.get_top(index)
                .and_then(|top_index| self.get_right(top_index)),
            self.get_bottom(index)
                .and_then(|bottom_index| self.get_left(bottom_index)),
            self.get_bottom(index)
                .and_then(|bottom_index| self.get_right(bottom_index)),
        ];

        for cell in directions {
            alive_count += is_alive(cell, &self.cell);
        }

        alive_count
    }

    pub fn game_logic(&self) {
        self.check_neighbours(24);
    }
}
impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                match self.cell[index] {
                    Cell::ALIVE => board.push_str("#"),
                    Cell::DEAD => board.push_str("."),
                }
            }
            board.push_str("\n")
        }

        write!(f, "{}", board)
    }
}

pub fn main() {
    let un = Universe::new(5, 5);
    un.game_logic();
    println!("{}", un)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_left() {
        let un = Universe::new(5, 5);
        assert_eq!(un.get_left(1), Some(0));
        assert_eq!(un.get_left(0), None);
        assert_eq!(un.get_left(5), None);
    }

    #[test]
    fn test_get_right() {
        let un = Universe::new(5, 5);
        assert_eq!(un.get_right(0), Some(1));
        assert_eq!(un.get_right(4), None);
        assert_eq!(un.get_right(9), None);
    }

    #[test]
    fn test_get_top() {
        let un = Universe::new(5, 5);
        assert_eq!(un.get_top(6), Some(1));
        assert_eq!(un.get_top(24), Some(19));
        assert_eq!(un.get_top(3), None);
        assert_eq!(un.get_top(0), None);
    }

    #[test]
    fn test_get_bottom() {
        let un = Universe::new(5, 5);
        assert_eq!(un.get_bottom(0), Some(5));
        assert_eq!(un.get_bottom(6), Some(11));
        assert_eq!(un.get_bottom(21), None);
        assert_eq!(un.get_bottom(24), None);
    }
}
