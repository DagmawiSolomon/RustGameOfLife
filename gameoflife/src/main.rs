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
    fn get_left(&self, index: Option<u32>) -> Option<u32>{
        match index {
            Some(index) => if index > 0{
                if (index % self.width) > 0{
                    Some(index - 1)
                } else {
                    None
                }
                } else {
                    panic!("Right: Index out of range");
                }
            None => {
                index
            }
        }
        
    }
    fn get_right(&self, index: Option<u32>) -> Option<u32>{
       
        match index {
            Some(index) =>  if index < self.width * self.height {
                if (index % self.width) < self.width - 1 {
                    Some(index + 1)
                } else {
                    None
                }
                } else {
                    panic!("Right: Index out of range");
                }
            None => {
                index
            }
        }
    }

    fn get_top(&self, index:u32) -> Option<u32>{
        if index > self.width{
            if index as i32 - self.width as i32 >= 0{
                Some(index - self.width)
            }
            else{
                None
            }
        } 
        else{
            panic!("Top: Index out of range");
            None
        }
    }
    fn get_bottom(&self, index:u32) -> Option<u32>{
        if index < self.height*self.width{
            if index  + self.width  < self.height*self.width{
                Some(index + self.width)
            }
            else{
                None
            }
        } 
        else{
            panic!("Bottom: Index out of range");
            None
        }
    }
    fn get_neighbours(&self, index: u32) {

        let left = self.get_left(Some(index));
        let right = self.get_right(Some(index));

        let top = self.get_top(index);
        let top_left = self.get_left(top);
        let top_right = self.get_right(top);

        let bottom = self.get_bottom(index);
        let bottom_left = self.get_left(bottom);
        let bottom_right = self.get_right(bottom);
        println!("{:?} {:?} {:?}",top_left, top, top_right);
        println!("{:?} {:?} {:?}",left,index,right);
        println!("{:?} {:?} {:?}",bottom_left, bottom, bottom_right);
        
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbours() {
   
    }
}
