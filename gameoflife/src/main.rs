#[derive(Debug)]
pub enum Cell{
    ALIVE,
    DEAD
}

pub struct Universe{
    width: u32,
    height: u32,
    cell: Vec<Cell>,
}
impl Universe{
    // try passing heigh as a value
    fn get_index(&self, x:u32, y:u32) -> usize{
        (y * self.height + x) as usize
    }
    pub fn new(width:u32, height:u32) -> Universe{
        let mut cells: Vec<Cell> = Vec::new();
        for i in 0 ..(width*height){
            cells.push(Cell::DEAD);
            if i % 31 == 0{
                cells.push(Cell::ALIVE);
            }
        }
        Universe{
            width:width,
            height:height,
            cell:cells,
        }

    }

    

    pub fn game_logic(&self){
        for i in 0 ..(self.width*self.height){
           match self.cell[i as usize]  {
               Cell::ALIVE => {
                 unimplemented!();
               }
               Cell::DEAD => continue
           }
        }
    }
}
impl std::fmt::Display for Universe{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..self.height{
            for x in 0..self.width{
                let index = self.get_index(x, y);
                match self.cell[index] {
                    Cell::ALIVE => board.push_str("#"),
                    Cell::DEAD => board.push_str(".")
                }
                
            }
           board.push_str("\n")
        }

        write!(f, "{}", board)
    }
}


pub fn main(){
    let un = Universe::new(70, 30);
    println!("{}", un)
}
