use std::fs;


// find xmas matches
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {

    fn new (contents: &str) -> Self {
        let data = contents.lines().map(|line| line.chars().collect()).collect();
        Grid { data }
    }

    fn find_word(&self, word: &str) -> i32 {
        let mut count = 0;
        
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                for direction in [
                    Direction::North,
                    Direction::South,
                    Direction::East,
                    Direction::West,
                    Direction::NorthEast,
                    Direction::NorthWest,
                    Direction::SouthEast,
                    Direction::SouthWest,
                ] {
                    if self.check_direction(x, y, &direction, word) {
                        count += 1;
                    } 
                }
            }
        }
        count
    }

    fn check_direction(&self, x: usize, y: usize, direction: &Direction, word: &str) -> bool {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (-1, -1),
            Direction::SouthEast => (1, 1),
            Direction::SouthWest => (-1, 1),
        };

        let mut current_x = x as isize;
        let mut current_y = y as isize;

        for c in word.chars() {
            // checking within bounds
            if current_x < 0 || current_y < 0 || current_y as usize >= self.data.len() || current_x as usize >= self.data[current_y as usize].len() {
                return false;
            }
            
            if self.data[current_y as usize][current_x as usize] != c {
                return false;
            }

            current_x += dx;
            current_y += dy;
        }

        true
    }
    fn check_cross(&self) -> i32 {

        let mut count = 0;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if x == 0 || y == 0 || y + 1 >= self.data.len() || x + 1 >= self.data[y].len() {
                continue;
                }
                
                let ne = self.data[y - 1][x + 1];
                let nw = self.data[y - 1][x - 1];
                let se = self.data[y + 1][x + 1];
                let sw = self.data[y + 1][x - 1];
                
                if self.data[y][x] != 'A' {
                    continue;
                }
                
                let found = match (ne, sw, nw, se) {
                    ('M', 'S','M', 'S') => true,
                    ('M', 'S','S', 'M') => true,
                    ('S', 'M','M', 'S') => true,
                    ('S', 'M','S', 'M') => true,
                    _ => false,
                };
                if found {
                    count += 1;
                }
            }
        }
    count

    }
}

fn main() {
    
    let contents = fs::read_to_string("input.txt").expect("failed read");
    // println!("{}", contents);

    let grid = Grid::new(&contents);
    let word = "XMAS";
    
    let count = grid.find_word(word);

    println!("XMAS: {:?}", count);

    let count = grid.check_cross();
    println!("cross: {:?}", count);

    

}
