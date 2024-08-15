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

#[derive(Debug)]
pub enum IndexError<'a> {
    OutOfBounds(&'a str),
}

impl Universe {
    // try passing heigh as a value
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

    fn get_left(&self, index: u32) -> Result<u32, IndexError> {
        if index > 0 {
            if (index % self.width) > 0 {
                Ok(index - 1)
            } else {
                Ok(u32::MAX)
            }
        } else {
            Err(IndexError::OutOfBounds("Left: Index out of range"))
        }
    }

    fn get_right(&self, index: u32) -> Result<u32, IndexError> {
        if index < self.width * self.height {
            if index != u32::MAX && (index % self.width) < self.width - 1 {
                Ok(index + 1)
            } else {
                Ok(u32::MAX)
            }
        } else {
            Err(IndexError::OutOfBounds("Index out of range"))
        }
    }

    fn get_top(&self, index: u32) -> Result<u32, IndexError> {
        if index > self.width {
            return Err(IndexError::OutOfBounds("Index out of range"));
        }
        if index as i32 - self.width as i32 >= 0 {
            Ok(index - self.width)
        } else {
            Ok(u32::MAX)
        }
    }

    fn get_bottom(&self, index: u32) -> Result<u32, IndexError> {
        if index >= self.width * self.height {
            return Err(IndexError::OutOfBounds("Index out of range"));
        }

        let new_index = index + self.width;

        if new_index >= self.width * self.height {
            Ok(u32::MAX)
        } else {
            Ok(new_index)
        }
    }

    fn get_neighbours(&self, index: u32) {
        let left = self.get_left(index);
        let right = self.get_right(index);

        let top = self.get_top(index);
        let top_left = self.get_left(index);
        let top_right = self.get_right(index);

        let bottom = self.get_bottom(index);
        let bottom_left = self.get_left(index);
        let bottom_right = self.get_right(index);
        println!("{:?} {:?} {:?}", top_left, top, top_right);
        println!("{:?} {:?} {:?}", left, index, right);
        println!("{:?} {:?} {:?}", bottom_left, bottom, bottom_right);
    }
    pub fn game_logic(&self) {
        self.get_neighbours(24);
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_get_left() {
//         let un = Universe::new(5, 5);
//         // Test left at index 1 (should return Some(0))
//         assert_eq!(un.get_left(Ok(Some(1)), Some(0));
//         // Test left at index 0 (should return None, as it's at the edge)
//         assert_eq!(un.get_left(Some(0)), None);
//         // Test left at index 5 (should return None, as it's at the start of the second row)
//         assert_eq!(un.get_left(Some(5)), None);
//         // Test None input (should return None)
//         assert_eq!(un.get_left(None), None);
//     }

//     #[test]
//     fn test_get_right() {
//         let un = Universe::new(5, 5);
//         // Test right at index 0 (should return Some(1))
//         assert_eq!(un.get_right(Some(0)), Some(1));
//         // Test right at index 4 (should return None, as it's at the edge)
//         assert_eq!(un.get_right(Some(4)), None);
//         // Test right at index 9 (should return None, as it's at the end of the second row)
//         assert_eq!(un.get_right(Some(9)), None);
//         // Test None input (should return None)
//         assert_eq!(un.get_right(Ok(None)), None);
//     }

//     #[test]
//     fn test_get_top() {
//         let un = Universe::new(5, 5);
//         // Test top at index 6 (should return Some(1))
//         assert_eq!(un.get_top(6), Some(1));
//         // Test top at index 1 (should panic as it's in the first row)
//         let result = std::panic::catch_unwind(|| un.get_top(1));
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_get_bottom() {
//         let un = Universe::new(5, 5);
//         // Test bottom at index 6 (should return Some(11))
//         assert_eq!(un.get_bottom(6), Some(11));
//         // Test bottom at index 21 (should return None as it's in the last row)
//         assert_eq!(un.get_bottom(21), None);
//         // Test bottom at an out-of-bound index (should panic)
//         let result = std::panic::catch_unwind(|| un.get_bottom(25));
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_get_neighbours() {
//         let un = Universe::new(5, 5);
//         // Test neighbours of a center cell (index 12)
//         let neighbours = vec![
//             un.get_left(Some(12)),
//             un.get_right(Some(12)),
//             un.get_top(12),
//             un.get_bottom(12),
//         ];
//         let expected_neighbours = vec![Some(11), Some(13), Some(7), Some(17)];
//         assert_eq!(neighbours, expected_neighbours);
//     }
// }
